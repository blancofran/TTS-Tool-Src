import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  DownloadProgress,
  ExportFormat,
  ModelKind,
  ModelStatus,
  TranscriptionProgress,
  TranscriptionResult,
} from "../types";

export function transcribeFile(
  filePath: string,
  mode: ModelKind,
): Promise<TranscriptionResult> {
  return invoke("transcribe_file", { filePath, mode });
}

export function cancelTranscription(): Promise<boolean> {
  return invoke("cancel_transcription");
}

export function getModelStatus(kind: ModelKind): Promise<ModelStatus> {
  return invoke("get_model_status", { kind });
}

export function downloadModel(kind: ModelKind): Promise<void> {
  return invoke("download_model", { kind });
}

export function exportTranscript(
  savePath: string,
  format: ExportFormat,
  text: string,
  segments: TranscriptionResult["segments"],
): Promise<void> {
  return invoke("export_transcript", { savePath, format, text, segments });
}

export function onTranscriptionProgress(
  callback: (progress: TranscriptionProgress) => void,
): Promise<UnlistenFn> {
  return listen<TranscriptionProgress>("transcription://progress", (event) =>
    callback(event.payload),
  );
}

export function onModelDownloadProgress(
  callback: (progress: DownloadProgress) => void,
): Promise<UnlistenFn> {
  return listen<DownloadProgress>("model://download-progress", (event) =>
    callback(event.payload),
  );
}
