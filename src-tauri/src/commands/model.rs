use crate::error::AppResult;
use crate::models::catalog::{model_info, ModelKind, ModelStatus};
use crate::models::{download, is_model_downloaded};
use crate::state::TranscriptionState;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn get_model_status(app: AppHandle, kind: ModelKind) -> AppResult<ModelStatus> {
    let info = model_info(kind);
    Ok(ModelStatus {
        kind,
        file_name: info.file_name.to_string(),
        downloaded: is_model_downloaded(&app, kind),
        approx_size_bytes: info.approx_size_bytes,
    })
}

/// Downloads the `high_accuracy` (medium) model on demand. The frontend is
/// expected to show a size warning and get user confirmation before calling this.
#[tauri::command]
pub async fn download_model(
    app: AppHandle,
    state: State<'_, TranscriptionState>,
    kind: ModelKind,
) -> AppResult<()> {
    let cancel_flag = state.begin_job();
    let result = download::download_model(&app, kind, &cancel_flag).await;
    state.end_job();
    result
}

/// Cancels the currently running transcription or model download, if any.
#[tauri::command]
pub async fn cancel_transcription(state: State<'_, TranscriptionState>) -> AppResult<bool> {
    Ok(state.request_cancel())
}
