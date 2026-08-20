export type ModelKind = "fast" | "high_accuracy";

export interface TranscriptSegment {
  startSec: number;
  endSec: number;
  text: string;
}

export interface TranscriptionResult {
  text: string;
  segments: TranscriptSegment[];
  language: string | null;
}

export type TranscriptionStage =
  | "downloading_model"
  | "extracting_audio"
  | "transcribing"
  | "completed";

export interface TranscriptionProgress {
  stage: TranscriptionStage;
  message: string;
}

export interface ModelStatus {
  kind: ModelKind;
  fileName: string;
  downloaded: boolean;
  approxSizeBytes: number;
}

export interface DownloadProgress {
  model: ModelKind;
  downloadedBytes: number;
  totalBytes: number | null;
}

export type ExportFormat = "txt" | "srt" | "vtt";

// Discriminated union describing the errors surfaced by Rust commands (see `error.rs`).
export interface AppErrorPayload {
  kind:
    | "invalid_input"
    | "ffmpeg_failed"
    | "model_error"
    | "model_not_downloaded"
    | "download_failed"
    | "checksum_mismatch"
    | "transcription_failed"
    | "io_error"
    | "cancelled";
  message: string;
}

export function isAppErrorPayload(err: unknown): err is AppErrorPayload {
  return (
    typeof err === "object" &&
    err !== null &&
    "kind" in err &&
    "message" in err
  );
}
