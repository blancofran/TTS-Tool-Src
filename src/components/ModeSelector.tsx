import type { ModelKind } from "../types";

interface ModeSelectorProps {
  mode: ModelKind;
  disabled?: boolean;
  onChange: (mode: ModelKind) => void;
}

export function ModeSelector({ mode, disabled, onChange }: ModeSelectorProps) {
  return (
    <div className="flex gap-3">
      <label
        className={`flex-1 cursor-pointer rounded-md border p-3 text-sm ${
          mode === "fast"
            ? "border-blue-500 bg-blue-50"
            : "border-slate-300 bg-white"
        } ${disabled ? "opacity-50" : ""}`}
      >
        <input
          type="radio"
          name="mode"
          className="mr-2"
          checked={mode === "fast"}
          disabled={disabled}
          onChange={() => onChange("fast")}
        />
        <span className="font-medium">Rápido</span>
        <p className="mt-1 text-xs text-slate-500">
          Modelo small, incluido en la app.
        </p>
      </label>
      <label
        className={`flex-1 cursor-pointer rounded-md border p-3 text-sm ${
          mode === "high_accuracy"
            ? "border-blue-500 bg-blue-50"
            : "border-slate-300 bg-white"
        } ${disabled ? "opacity-50" : ""}`}
      >
        <input
          type="radio"
          name="mode"
          className="mr-2"
          checked={mode === "high_accuracy"}
          disabled={disabled}
          onChange={() => onChange("high_accuracy")}
        />
        <span className="font-medium">Precisión alta</span>
        <p className="mt-1 text-xs text-slate-500">
          Modelo medium, se descarga la primera vez (~500-800 MB).
        </p>
      </label>
    </div>
  );
}
