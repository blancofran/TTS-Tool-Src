use crate::error::{AppError, AppResult};
use crate::state::is_cancelled;
use serde::Serialize;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSegment {
    pub start_sec: f64,
    pub end_sec: f64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionResult {
    pub text: String,
    pub segments: Vec<TranscriptSegment>,
    /// ISO-639-1-ish code as reported by whisper.cpp's language detector, if any.
    pub language: Option<String>,
}

/// Reads a mono 16kHz PCM16 WAV file into the `f32` samples whisper.cpp expects.
fn read_wav_samples(path: &Path) -> AppResult<Vec<f32>> {
    let mut reader =
        hound::WavReader::open(path).map_err(|e| AppError::TranscriptionFailed(e.to_string()))?;
    let spec = reader.spec();
    if spec.channels != 1 || spec.sample_rate != 16000 {
        return Err(AppError::TranscriptionFailed(format!(
            "expected mono 16kHz WAV, got {} channel(s) at {}Hz",
            spec.channels, spec.sample_rate
        )));
    }

    let samples: Result<Vec<f32>, _> = match spec.sample_format {
        hound::SampleFormat::Int => reader
            .samples::<i16>()
            .map(|s| s.map(|v| v as f32 / i16::MAX as f32))
            .collect(),
        hound::SampleFormat::Float => reader.samples::<f32>().collect(),
    };
    samples.map_err(|e| AppError::TranscriptionFailed(e.to_string()))
}

/// Runs whisper.cpp on `wav_path` using the model at `model_path`, with
/// automatic language detection (no language is forced).
pub fn transcribe(
    model_path: &Path,
    wav_path: &Path,
    cancel_flag: &Arc<AtomicBool>,
) -> AppResult<TranscriptionResult> {
    if !model_path.is_file() {
        return Err(AppError::ModelError(format!(
            "model file not found: {}",
            model_path.display()
        )));
    }

    let samples = read_wav_samples(wav_path)?;

    let ctx = WhisperContext::new_with_params(
        &model_path.to_string_lossy(),
        WhisperContextParameters::default(),
    )
    .map_err(|e| AppError::ModelError(e.to_string()))?;
    let mut state = ctx
        .create_state()
        .map_err(|e| AppError::ModelError(e.to_string()))?;

    // whisper.cpp's ggml CPU backend can intermittently fail to encode
    // ("error code: -6") under some thread counts; retry once single-threaded,
    // which is the most robust (if slowest) configuration, before giving up.
    match run_full(&mut state, &samples, cancel_flag, num_cpus()) {
        Ok(()) => {}
        Err(AppError::Cancelled) => return Err(AppError::Cancelled),
        Err(_first_err) => {
            run_full(&mut state, &samples, cancel_flag, 1)?;
        }
    }

    let num_segments = state
        .full_n_segments()
        .map_err(|e| AppError::TranscriptionFailed(e.to_string()))?;

    let mut segments = Vec::with_capacity(num_segments as usize);
    let mut full_text = String::new();
    for i in 0..num_segments {
        let text = state
            .full_get_segment_text(i)
            .map_err(|e| AppError::TranscriptionFailed(e.to_string()))?;
        let t0 = state
            .full_get_segment_t0(i)
            .map_err(|e| AppError::TranscriptionFailed(e.to_string()))?;
        let t1 = state
            .full_get_segment_t1(i)
            .map_err(|e| AppError::TranscriptionFailed(e.to_string()))?;

        full_text.push_str(text.trim());
        full_text.push(' ');
        segments.push(TranscriptSegment {
            start_sec: t0 as f64 / 100.0, // whisper.cpp reports centiseconds
            end_sec: t1 as f64 / 100.0,
            text: text.trim().to_string(),
        });
    }

    let language = state
        .full_lang_id_from_state()
        .ok()
        .and_then(whisper_rs::get_lang_str)
        .map(|s| s.to_string());

    Ok(TranscriptionResult {
        text: full_text.trim().to_string(),
        segments,
        language,
    })
}

fn num_cpus() -> std::ffi::c_int {
    // whisper.cpp's own CLI defaults to 4 threads; higher counts have been
    // known to trigger intermittent ggml CPU-backend encode failures.
    std::thread::available_parallelism()
        .map(|n| n.get().min(4) as std::ffi::c_int)
        .unwrap_or(4)
}

fn run_full(
    state: &mut whisper_rs::WhisperState,
    samples: &[f32],
    cancel_flag: &Arc<AtomicBool>,
    n_threads: std::ffi::c_int,
) -> AppResult<()> {
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_print_progress(false);
    params.set_print_special(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_language(None); // auto-detect
    params.set_n_threads(n_threads);

    let abort_flag = cancel_flag.clone();
    params.set_abort_callback_safe(move || is_cancelled(&abort_flag));

    state
        .full(params, samples)
        .map(|_| ())
        .map_err(|e| {
            if is_cancelled(cancel_flag) {
                AppError::Cancelled
            } else {
                AppError::TranscriptionFailed(e.to_string())
            }
        })
}
