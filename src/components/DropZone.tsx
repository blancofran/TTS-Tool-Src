import { useEffect, useRef, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWebview } from "@tauri-apps/api/webview";

const ACCEPTED_EXTENSIONS = [
  "mp3",
  "wav",
  "m4a",
  "flac",
  "ogg",
  "mp4",
  "mkv",
  "mov",
  "avi",
];

interface DropZoneProps {
  selectedPath: string | null;
  disabled?: boolean;
  onFileSelected: (path: string) => void;
}

function hasAcceptedExtension(path: string): boolean {
  const ext = path.split(".").pop()?.toLowerCase();
  return !!ext && ACCEPTED_EXTENSIONS.includes(ext);
}

export function DropZone({ selectedPath, disabled, onFileSelected }: DropZoneProps) {
  const [isDraggingOver, setIsDraggingOver] = useState(false);
  const disabledRef = useRef(disabled);
  disabledRef.current = disabled;

  useEffect(() => {
    const unlistenPromise = getCurrentWebview().onDragDropEvent((event) => {
      if (disabledRef.current) return;

      if (event.payload.type === "over") {
        setIsDraggingOver(true);
      } else if (event.payload.type === "drop") {
        setIsDraggingOver(false);
        const path = event.payload.paths[0];
        if (path && hasAcceptedExtension(path)) {
          onFileSelected(path);
        }
      } else {
        setIsDraggingOver(false);
      }
    });
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [onFileSelected]);

  async function handleBrowseClick() {
    const path = await open({
      multiple: false,
      filters: [
        {
          name: "Audio / Video",
          extensions: ACCEPTED_EXTENSIONS,
        },
      ],
    });
    if (typeof path === "string") {
      onFileSelected(path);
    }
  }

  return (
    <div
      className={`flex flex-col items-center justify-center gap-3 rounded-lg border-2 border-dashed p-10 text-center transition-colors ${
        isDraggingOver
          ? "border-blue-500 bg-blue-50"
          : "border-slate-300 bg-slate-50"
      } ${disabled ? "opacity-50" : ""}`}
    >
      {selectedPath ? (
        <p className="break-all text-sm font-medium text-slate-700">
          {selectedPath}
        </p>
      ) : (
        <p className="text-sm text-slate-500">
          Arrastra un archivo de audio o video aquí, o
        </p>
      )}
      <button
        type="button"
        onClick={handleBrowseClick}
        disabled={disabled}
        className="rounded-md bg-slate-800 px-4 py-2 text-sm font-medium text-white hover:bg-slate-700 disabled:cursor-not-allowed"
      >
        Seleccionar archivo
      </button>
      <p className="text-xs text-slate-400">
        mp3, wav, m4a, flac, ogg, mp4, mkv, mov, avi
      </p>
    </div>
  );
}
