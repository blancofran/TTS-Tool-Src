use crate::error::AppResult;
use crate::whisper::engine::TranscriptSegment;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Txt,
    Srt,
    Vtt,
}

fn format_timestamp(total_seconds: f64, decimal_sep: char) -> String {
    let millis_total = (total_seconds * 1000.0).round() as i64;
    let hours = millis_total / 3_600_000;
    let minutes = (millis_total % 3_600_000) / 60_000;
    let seconds = (millis_total % 60_000) / 1000;
    let millis = millis_total % 1000;
    format!(
        "{:02}:{:02}:{:02}{}{:03}",
        hours, minutes, seconds, decimal_sep, millis
    )
}

fn to_srt(segments: &[TranscriptSegment]) -> String {
    let mut out = String::new();
    for (i, seg) in segments.iter().enumerate() {
        out.push_str(&format!("{}\n", i + 1));
        out.push_str(&format!(
            "{} --> {}\n",
            format_timestamp(seg.start_sec, ','),
            format_timestamp(seg.end_sec, ',')
        ));
        out.push_str(seg.text.trim());
        out.push_str("\n\n");
    }
    out
}

fn to_vtt(segments: &[TranscriptSegment]) -> String {
    let mut out = String::from("WEBVTT\n\n");
    for seg in segments {
        out.push_str(&format!(
            "{} --> {}\n",
            format_timestamp(seg.start_sec, '.'),
            format_timestamp(seg.end_sec, '.')
        ));
        out.push_str(seg.text.trim());
        out.push_str("\n\n");
    }
    out
}

#[tauri::command]
pub async fn export_transcript(
    save_path: String,
    format: ExportFormat,
    text: String,
    segments: Vec<TranscriptSegment>,
) -> AppResult<()> {
    let content = match format {
        ExportFormat::Txt => text,
        ExportFormat::Srt => to_srt(&segments),
        ExportFormat::Vtt => to_vtt(&segments),
    };
    std::fs::write(save_path, content)?;
    Ok(())
}
