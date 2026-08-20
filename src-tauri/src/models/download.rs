use crate::error::{AppError, AppResult};
use crate::state::is_cancelled;
use futures_util::StreamExt;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::sync::atomic::AtomicBool;
use tauri::{AppHandle, Emitter};

use super::catalog::{model_info, ModelKind};
use super::{is_model_downloaded, resolve_model_path};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub model: ModelKind,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
}

const DOWNLOAD_PROGRESS_EVENT: &str = "model://download-progress";

/// Downloads `kind` (only [`ModelKind::HighAccuracy`] supports this) into the app
/// data dir, emitting `model://download-progress` events as it goes.
///
/// whisper.cpp's HuggingFace repo does not publish official per-file checksums,
/// so integrity is verified on a trust-on-first-use basis: we compute the
/// SHA-256 of the freshly downloaded file and store it next to it. Later runs
/// re-verify against that stored hash to detect a corrupted local copy, not to
/// authenticate the upstream file itself.
pub async fn download_model(
    app: &AppHandle,
    kind: ModelKind,
    cancel_flag: &AtomicBool,
) -> AppResult<()> {
    if kind == ModelKind::Fast {
        return Err(AppError::ModelError(
            "the fast model is bundled and cannot be downloaded".into(),
        ));
    }
    if is_model_downloaded(app, kind) {
        return Ok(());
    }

    let info = model_info(kind);
    let url = info
        .download_url
        .ok_or_else(|| AppError::ModelError("model has no download URL".into()))?;
    let dest = resolve_model_path(app, kind)?;
    let tmp_dest = dest.with_extension("part");

    let response = reqwest::get(url)
        .await
        .map_err(|e| AppError::DownloadFailed(e.to_string()))?;
    if !response.status().is_success() {
        return Err(AppError::DownloadFailed(format!(
            "server responded with {}",
            response.status()
        )));
    }
    let total_bytes = response.content_length();

    let mut file = std::fs::File::create(&tmp_dest)?;
    let mut hasher = Sha256::new();
    let mut downloaded_bytes: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        if is_cancelled(cancel_flag) {
            drop(file);
            let _ = std::fs::remove_file(&tmp_dest);
            return Err(AppError::Cancelled);
        }
        let chunk = chunk.map_err(|e| AppError::DownloadFailed(e.to_string()))?;
        file.write_all(&chunk)?;
        hasher.update(&chunk);
        downloaded_bytes += chunk.len() as u64;

        let _ = app.emit(
            DOWNLOAD_PROGRESS_EVENT,
            DownloadProgress {
                model: kind,
                downloaded_bytes,
                total_bytes,
            },
        );
    }
    file.sync_all()?;
    drop(file);

    let hash = format!("{:x}", hasher.finalize());
    std::fs::write(dest.with_extension("sha256"), &hash)?;
    std::fs::rename(&tmp_dest, &dest)?;

    Ok(())
}

/// Re-checks a previously downloaded model against the hash captured at
/// download time. Returns `Ok(true)` if the file is missing (nothing to verify).
pub fn verify_cached_model(app: &AppHandle, kind: ModelKind) -> AppResult<bool> {
    let dest = resolve_model_path(app, kind)?;
    if !dest.is_file() {
        return Ok(true);
    }
    let hash_path = dest.with_extension("sha256");
    let expected = match std::fs::read_to_string(&hash_path) {
        Ok(h) => h.trim().to_string(),
        Err(_) => return Ok(true), // no pinned hash captured (e.g. manually placed file)
    };

    let mut file = std::fs::File::open(&dest)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let actual = format!("{:x}", hasher.finalize());

    Ok(actual == expected)
}
