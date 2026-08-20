use serde::Serialize;

/// Application-level error, serialized to the frontend as `{ kind, message }`.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Unsupported or unreadable input file: {0}")]
    InvalidInput(String),

    #[error("Audio extraction failed: {0}")]
    FfmpegFailed(String),

    #[error("Whisper model error: {0}")]
    ModelError(String),

    #[error("The '{0}' model is not downloaded yet")]
    ModelNotDownloaded(String),

    #[error("Model download failed: {0}")]
    DownloadFailed(String),

    #[error("Downloaded model failed integrity check (SHA-256 mismatch)")]
    ChecksumMismatch,

    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("Cancelled by the user")]
    Cancelled,
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e.to_string())
    }
}

/// Mirrors [`AppError`] in a shape that's convenient to match on in the frontend.
#[derive(Serialize)]
struct AppErrorPayload {
    kind: &'static str,
    message: String,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind = match self {
            AppError::InvalidInput(_) => "invalid_input",
            AppError::FfmpegFailed(_) => "ffmpeg_failed",
            AppError::ModelError(_) => "model_error",
            AppError::ModelNotDownloaded(_) => "model_not_downloaded",
            AppError::DownloadFailed(_) => "download_failed",
            AppError::ChecksumMismatch => "checksum_mismatch",
            AppError::TranscriptionFailed(_) => "transcription_failed",
            AppError::Io(_) => "io_error",
            AppError::Cancelled => "cancelled",
        };
        AppErrorPayload {
            kind,
            message: self.to_string(),
        }
        .serialize(serializer)
    }
}

pub type AppResult<T> = Result<T, AppError>;
