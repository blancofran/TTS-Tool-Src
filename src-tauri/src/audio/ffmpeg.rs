use crate::error::{AppError, AppResult};
use crate::state::is_cancelled;
use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::download::auto_download;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Once;
use std::time::Duration;

static FFMPEG_READY: Once = Once::new();

/// Makes sure a working `ffmpeg` binary is available, downloading one
/// (via ffmpeg-sidecar) the first time it's needed.
fn ensure_ffmpeg() -> AppResult<()> {
    let mut result = Ok(());
    FFMPEG_READY.call_once(|| {
        if let Err(e) = auto_download() {
            result = Err(AppError::FfmpegFailed(format!(
                "could not obtain ffmpeg binary: {e}"
            )));
        }
    });
    result
}

/// Converts any audio/video input into a 16kHz mono PCM16 WAV file, the format
/// whisper.cpp requires. Returns the path to the generated WAV (in a temp dir).
pub fn extract_wav_16k_mono(input: &Path, cancel_flag: &AtomicBool) -> AppResult<PathBuf> {
    ensure_ffmpeg()?;

    if !input.is_file() {
        return Err(AppError::InvalidInput(format!(
            "file not found: {}",
            input.display()
        )));
    }

    let tmp_dir = tempfile::Builder::new()
        .prefix("tts-tool-")
        .tempdir()
        .map_err(AppError::from)?
        // Detach from the `TempDir` guard: the caller deletes the parent dir
        // explicitly once transcription has consumed the WAV.
        .keep();
    let out_path_owned = tmp_dir.join("audio-16k-mono.wav");

    let mut child = FfmpegCommand::new()
        .input(input.to_string_lossy().to_string())
        .args(["-vn", "-ac", "1", "-ar", "16000", "-f", "wav"])
        .output(out_path_owned.to_string_lossy().to_string())
        .spawn()
        .map_err(|e| AppError::FfmpegFailed(e.to_string()))?;

    // Poll instead of blocking on `wait()` so a cancellation can kill ffmpeg promptly.
    let status = loop {
        if is_cancelled(cancel_flag) {
            let _ = child.kill();
            let _ = std::fs::remove_dir_all(&tmp_dir);
            return Err(AppError::Cancelled);
        }
        match child
            .as_inner_mut()
            .try_wait()
            .map_err(|e| AppError::FfmpegFailed(e.to_string()))?
        {
            Some(status) => break status,
            None => std::thread::sleep(Duration::from_millis(150)),
        }
    };

    if !status.success() {
        return Err(AppError::FfmpegFailed(format!(
            "ffmpeg exited with status {status}"
        )));
    }
    if !out_path_owned.is_file() {
        return Err(AppError::FfmpegFailed(
            "ffmpeg did not produce an output file".into(),
        ));
    }

    Ok(out_path_owned)
}

