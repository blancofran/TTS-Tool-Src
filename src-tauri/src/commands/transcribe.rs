use crate::audio::ffmpeg::extract_wav_16k_mono;
use crate::error::{AppError, AppResult};
use crate::models::catalog::ModelKind;
use crate::models::{download, is_model_downloaded, resolve_model_path};
use crate::state::{is_cancelled, TranscriptionState};
use crate::whisper::engine::{self, TranscriptionResult};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

const PROGRESS_EVENT: &str = "transcription://progress";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
enum Stage {
    DownloadingModel,
    ExtractingAudio,
    Transcribing,
    Completed,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProgressPayload {
    stage: Stage,
    message: String,
}

fn emit_progress(app: &AppHandle, stage: Stage, message: &str) {
    let _ = app.emit(
        PROGRESS_EVENT,
        ProgressPayload {
            stage,
            message: message.to_string(),
        },
    );
}

#[tauri::command]
pub async fn transcribe_file(
    app: AppHandle,
    state: State<'_, TranscriptionState>,
    file_path: String,
    mode: ModelKind,
) -> AppResult<TranscriptionResult> {
    let cancel_flag = state.begin_job();
    let result = run_transcription(&app, &cancel_flag, file_path, mode).await;
    state.end_job();
    result
}

async fn run_transcription(
    app: &AppHandle,
    cancel_flag: &Arc<AtomicBool>,
    file_path: String,
    mode: ModelKind,
) -> AppResult<TranscriptionResult> {
    if !is_model_downloaded(app, mode) {
        match mode {
            ModelKind::Fast => {
                return Err(AppError::ModelNotDownloaded("small".to_string()));
            }
            ModelKind::HighAccuracy => {
                emit_progress(
                    app,
                    Stage::DownloadingModel,
                    "Descargando modelo de precisión alta (medium)...",
                );
                download::download_model(app, mode, cancel_flag).await?;
            }
        }
    }
    if is_cancelled(cancel_flag) {
        return Err(AppError::Cancelled);
    }

    let model_path = resolve_model_path(app, mode)?;
    let input_path = PathBuf::from(file_path);

    emit_progress(app, Stage::ExtractingAudio, "Extrayendo audio...");
    let app_for_blocking = app.clone();
    let cancel_for_blocking = cancel_flag.clone();
    let result = tauri::async_runtime::spawn_blocking(move || -> AppResult<TranscriptionResult> {
        let wav_path = extract_wav_16k_mono(&input_path, &cancel_for_blocking)?;
        emit_progress(
            &app_for_blocking,
            Stage::Transcribing,
            "Transcribiendo... (puede tardar varios minutos)",
        );
        let transcription = engine::transcribe(&model_path, &wav_path, &cancel_for_blocking);

        // Best-effort cleanup of the temp dir created for the extracted WAV.
        if let Some(parent) = wav_path.parent() {
            let _ = std::fs::remove_dir_all(parent);
        }

        transcription
    })
    .await
    .map_err(|e| AppError::TranscriptionFailed(e.to_string()))??;

    emit_progress(app, Stage::Completed, "Completado");
    Ok(result)
}
