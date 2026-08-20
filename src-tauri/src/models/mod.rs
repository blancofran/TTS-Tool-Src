pub mod catalog;
pub mod download;

use crate::error::{AppError, AppResult};
use catalog::{model_info, ModelKind};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// Directory (inside the app data dir) where on-demand models are stored.
fn models_dir(app: &AppHandle) -> AppResult<PathBuf> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Io(e.to_string()))?
        .join("models");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Resolves the on-disk path for a model, without checking it exists.
pub fn resolve_model_path(app: &AppHandle, kind: ModelKind) -> AppResult<PathBuf> {
    let info = model_info(kind);
    match kind {
        ModelKind::Fast => app
            .path()
            .resolve(
                format!("resources/models/{}", info.file_name),
                tauri::path::BaseDirectory::Resource,
            )
            .map_err(|e| AppError::ModelError(e.to_string())),
        ModelKind::HighAccuracy => Ok(models_dir(app)?.join(info.file_name)),
    }
}

pub fn is_model_downloaded(app: &AppHandle, kind: ModelKind) -> bool {
    resolve_model_path(app, kind)
        .map(|p| p.is_file())
        .unwrap_or(false)
}
