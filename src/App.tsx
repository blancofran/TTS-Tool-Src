import { useCallback, useEffect, useState } from "react";
import { DropZone } from "./components/DropZone";
import { ModeSelector } from "./components/ModeSelector";
import { ProgressBar } from "./components/ProgressBar";
import { TranscriptView } from "./components/TranscriptView";
import {
  cancelTranscription,
  onTranscriptionProgress,
  transcribeFile,
} from "./lib/tauri";
import {
  isAppErrorPayload,
  type ModelKind,
  type TranscriptionProgress,
  type TranscriptionResult,
} from "./types";

type Status = "idle" | "processing" | "done" | "error" | "cancelled";

function App() {
  const [filePath, setFilePath] = useState<string | null>(null);
  const [mode, setMode] = useState<ModelKind>("fast");
  const [status, setStatus] = useState<Status>("idle");
  const [progressMessage, setProgressMessage] = useState("");
  const [result, setResult] = useState<TranscriptionResult | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [isCancelling, setIsCancelling] = useState(false);

  useEffect(() => {
    const unlistenPromise = onTranscriptionProgress(
      (progress: TranscriptionProgress) => {
        setProgressMessage(progress.message);
      },
    );
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const handleFileSelected = useCallback((path: string) => {
    setFilePath(path);
    setResult(null);
    setErrorMessage(null);
    setStatus("idle");
  }, []);

  async function handleTranscribe() {
    if (!filePath) return;
    setStatus("processing");
    setErrorMessage(null);
    setResult(null);
    setIsCancelling(false);
    try {
      const transcription = await transcribeFile(filePath, mode);
      setResult(transcription);
      setStatus("done");
    } catch (err) {
      if (isAppErrorPayload(err) && err.kind === "cancelled") {
        setStatus("cancelled");
      } else {
        setErrorMessage(
          isAppErrorPayload(err)
            ? err.message
            : "Ocurrió un error inesperado.",
        );
        setStatus("error");
      }
    } finally {
      setIsCancelling(false);
    }
  }

  async function handleCancel() {
    setIsCancelling(true);
    setProgressMessage("Cancelando...");
    try {
      await cancelTranscription();
    } catch {
      setIsCancelling(false);
    }
  }

  const isProcessing = status === "processing";

  return (
    <main className="mx-auto flex min-h-screen max-w-2xl flex-col gap-6 bg-white p-8">
      <header>
        <h1 className="text-xl font-bold text-slate-900">TTS Tool</h1>
        <p className="text-sm text-slate-500">
          Transcribe audio y video localmente con Whisper, sin conexión a
          internet.
        </p>
      </header>

      <DropZone
        selectedPath={filePath}
        disabled={isProcessing}
        onFileSelected={handleFileSelected}
      />

      <ModeSelector mode={mode} disabled={isProcessing} onChange={setMode} />

      <div className="flex gap-3">
        <button
          type="button"
          onClick={handleTranscribe}
          disabled={!filePath || isProcessing}
          className="flex-1 rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white hover:bg-blue-500 disabled:cursor-not-allowed disabled:bg-slate-300"
        >
          {isProcessing ? "Transcribiendo..." : "Transcribir"}
        </button>

        {isProcessing && (
          <button
            type="button"
            onClick={handleCancel}
            disabled={isCancelling}
            className="rounded-md border border-red-300 px-4 py-2 text-sm font-semibold text-red-700 hover:bg-red-50 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {isCancelling ? "Cancelando..." : "Cancelar"}
          </button>
        )}
      </div>

      {isProcessing && <ProgressBar message={progressMessage} />}

      {status === "cancelled" && (
        <p className="rounded-md bg-slate-100 p-3 text-sm text-slate-600">
          Transcripción cancelada.
        </p>
      )}

      {status === "error" && errorMessage && (
        <div className="rounded-md bg-red-50 p-3 text-sm text-red-700">
          <p>{errorMessage}</p>
          <p className="mt-1 text-xs text-red-500">
            Si el error persiste, intenta de nuevo, con un archivo más corto o
            cerrando otras aplicaciones para liberar memoria.
          </p>
        </div>
      )}

      {result && <TranscriptView result={result} />}
    </main>
  );
}

export default App;
