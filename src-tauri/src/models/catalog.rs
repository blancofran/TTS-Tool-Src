use serde::Serialize;

/// Which Whisper model/mode the user selected in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelKind {
    /// `ggml-small-q5_1.bin`, bundled inside the installer as a Tauri resource.
    Fast,
    /// `ggml-medium-q5_0.bin`, downloaded on demand into the app data dir.
    HighAccuracy,
}

pub struct ModelInfo {
    pub kind: ModelKind,
    pub file_name: &'static str,
    /// `None` for bundled models (resolved as a Tauri resource, not downloaded).
    pub download_url: Option<&'static str>,
    pub approx_size_bytes: u64,
}

pub const FAST_MODEL: ModelInfo = ModelInfo {
    kind: ModelKind::Fast,
    file_name: "ggml-small-q5_1.bin",
    download_url: None,
    approx_size_bytes: 190 * 1024 * 1024,
};

pub const HIGH_ACCURACY_MODEL: ModelInfo = ModelInfo {
    kind: ModelKind::HighAccuracy,
    file_name: "ggml-medium-q5_0.bin",
    download_url: Some(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium-q5_0.bin",
    ),
    approx_size_bytes: 539 * 1024 * 1024,
};

pub fn model_info(kind: ModelKind) -> &'static ModelInfo {
    match kind {
        ModelKind::Fast => &FAST_MODEL,
        ModelKind::HighAccuracy => &HIGH_ACCURACY_MODEL,
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelStatus {
    pub kind: ModelKind,
    pub file_name: String,
    pub downloaded: bool,
    pub approx_size_bytes: u64,
}
