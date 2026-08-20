import { save } from "@tauri-apps/plugin-dialog";
import { exportTranscript } from "../lib/tauri";
import type { ExportFormat, TranscriptionResult } from "../types";

interface TranscriptViewProps {
  result: TranscriptionResult;
}

const EXPORTS: { format: ExportFormat; label: string; extensions: string[] }[] = [
  { format: "txt", label: "Exportar .txt", extensions: ["txt"] },
  { format: "srt", label: "Exportar .srt", extensions: ["srt"] },
  { format: "vtt", label: "Exportar .vtt", extensions: ["vtt"] },
];

export function TranscriptView({ result }: TranscriptViewProps) {
  async function handleExport(format: ExportFormat, extensions: string[]) {
    const savePath = await save({
      filters: [{ name: format.toUpperCase(), extensions }],
      defaultPath: `transcripcion.${extensions[0]}`,
    });
    if (!savePath) return;
    await exportTranscript(savePath, format, result.text, result.segments);
  }

  return (
    <div className="flex flex-col gap-3">
      <div className="flex items-center justify-between">
        <h2 className="text-sm font-semibold text-slate-700">
          Transcripción {result.language ? `(idioma detectado: ${result.language})` : ""}
        </h2>
        <div className="flex gap-2">
          {EXPORTS.map(({ format, label, extensions }) => (
            <button
              key={format}
              type="button"
              onClick={() => handleExport(format, extensions)}
              className="rounded-md border border-slate-300 px-3 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-100"
            >
              {label}
            </button>
          ))}
        </div>
      </div>
      <textarea
        readOnly
        value={result.text}
        className="h-64 w-full resize-none rounded-md border border-slate-300 p-3 text-sm text-slate-800"
      />
    </div>
  );
}
