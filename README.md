# TTS Tool

🇬🇧 English | [🇪🇸 Español](README.es.md)

Desktop app (Tauri 2 + React/TypeScript + Rust) that transcribes audio/video
files locally with Whisper (`whisper-rs` / whisper.cpp), no cloud backend and
no internet required for the core flow.

## Stack

- **App shell**: Tauri 2 (Rust backend, OS webview frontend).
- **Frontend**: React + TypeScript + Tailwind CSS, via `npm`.
- **Transcription**: `whisper-rs` (native bindings to whisper.cpp), CPU only.
- **Audio extraction**: `ffmpeg-sidecar`, converts any input to 16kHz mono WAV.
- **Bundled model**: `ggml-small-q5_1.bin` (~190MB), shipped as a Tauri resource.
- **On-demand model**: `ggml-medium-q5_0.bin` (~539MB), downloaded into the
  app data dir the first time "Precisión alta" is used.

## Downloads

Latest release: **[v0.1.0](https://github.com/blancofran/TTS-Tool-Src/releases/tag/v0.1.0)**

| Platform | File |
| --- | --- |
| macOS (Apple Silicon) | [`TTS Tool_0.1.0_aarch64.dmg`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_aarch64.dmg) |
| Windows | [`TTS Tool_0.1.0_x64-setup.exe`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_x64-setup.exe) or [`.msi`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_x64_en-US.msi) |
| Linux (Debian/Ubuntu) | [`TTS Tool_0.1.0_amd64.deb`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_amd64.deb) |
| Linux (Fedora/openSUSE) | [`TTS Tool-0.1.0-1.x86_64.rpm`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool-0.1.0-1.x86_64.rpm) |
| Linux (any distro, needs FUSE2) | [`TTS Tool_0.1.0_amd64.AppImage`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_amd64.AppImage) |

All versions: [Releases page](https://github.com/blancofran/TTS-Tool-Src/releases).
New builds land as **drafts** and are only public once reviewed and published
manually.

## Getting started

```bash
npm install
scripts/download-models.sh small   # or download-models.ps1 on Windows
npm run tauri dev
```

The `small` model must exist at `src-tauri/resources/models/ggml-small-q5_1.bin`
before running `dev` or `build` — it's a bundled Tauri resource, not fetched
at runtime. It's gitignored; each dev machine/CI run downloads it via the
script above.

## Project layout

```
src/                        React frontend
  components/                DropZone, ModeSelector, ProgressBar, TranscriptView
  lib/tauri.ts                invoke()/listen() wrappers for Rust commands
  types.ts                    shared TS types mirroring Rust DTOs

src-tauri/src/
  commands/                  #[tauri::command] entry points (transcribe, model, export)
  audio/ffmpeg.rs             input -> 16kHz mono WAV via ffmpeg-sidecar
  whisper/engine.rs           WAV -> transcript + timestamped segments
  models/                     model catalog, path resolution, on-demand download
  error.rs                    AppError, serialized to the frontend as {kind, message}

scripts/                     download-models.sh / .ps1 (fetch GGML models + pin SHA-256)
```

## Notes / known limitations (phase 1)

- ggerganov/whisper.cpp's HuggingFace repo does not publish official
  checksums for these files. Integrity is verified trust-on-first-use: the
  SHA-256 is computed right after downloading and stored next to the model,
  then re-checked on later runs to catch local corruption (not to
  authenticate the upstream file).
- Language is always auto-detected; there's no language selector yet.
- `.srt`/`.vtt` export and the medium-model download flow are implemented in
  the Rust backend but not yet wired into a "high accuracy first-run" UX
  (size warning + confirmation dialog before downloading).

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
