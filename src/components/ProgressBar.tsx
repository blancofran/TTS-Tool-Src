interface ProgressBarProps {
  message: string;
  indeterminate?: boolean;
  progress?: number; // 0..1
}

export function ProgressBar({ message, indeterminate = true, progress }: ProgressBarProps) {
  return (
    <div className="flex flex-col gap-2">
      <p className="text-sm text-slate-600">{message}</p>
      <div className="h-2 w-full overflow-hidden rounded-full bg-slate-200">
        {indeterminate ? (
          <div className="h-full w-1/3 animate-pulse rounded-full bg-blue-500" />
        ) : (
          <div
            className="h-full rounded-full bg-blue-500 transition-all"
            style={{ width: `${Math.round((progress ?? 0) * 100)}%` }}
          />
        )}
      </div>
    </div>
  );
}
