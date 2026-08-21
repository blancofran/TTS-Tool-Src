# TTS Tool

[🇬🇧 English](README.md) | 🇪🇸 Español

Aplicación de escritorio (Tauri 2 + React/TypeScript + Rust) que transcribe
archivos de audio/video localmente con Whisper (`whisper-rs` / whisper.cpp),
sin backend en la nube y sin necesidad de internet para el flujo principal.

## Stack

- **Shell de la app**: Tauri 2 (backend en Rust, webview del sistema operativo).
- **Frontend**: React + TypeScript + Tailwind CSS, vía `npm`.
- **Transcripción**: `whisper-rs` (bindings nativos a whisper.cpp), solo CPU.
- **Extracción de audio**: `ffmpeg-sidecar`, convierte cualquier entrada a WAV mono 16kHz.
- **Modelo incluido**: `ggml-small-q5_1.bin` (~190MB), incluido como recurso de Tauri.
- **Modelo bajo demanda**: `ggml-medium-q5_0.bin` (~539MB), se descarga al
  directorio de datos de la app la primera vez que se usa "Precisión alta".

## Descargas

Última versión: **[v0.1.0](https://github.com/blancofran/TTS-Tool-Src/releases/tag/v0.1.0)**

| Plataforma | Archivo |
| --- | --- |
| macOS (Apple Silicon) | [`TTS Tool_0.1.0_aarch64.dmg`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_aarch64.dmg) |
| Windows | [`TTS Tool_0.1.0_x64-setup.exe`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_x64-setup.exe) o [`.msi`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_x64_en-US.msi) |
| Linux (Debian/Ubuntu) | [`TTS Tool_0.1.0_amd64.deb`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_amd64.deb) |
| Linux (Fedora/openSUSE) | [`TTS Tool-0.1.0-1.x86_64.rpm`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool-0.1.0-1.x86_64.rpm) |
| Linux (cualquier distro, requiere FUSE2) | [`TTS Tool_0.1.0_amd64.AppImage`](https://github.com/blancofran/TTS-Tool-Src/releases/download/v0.1.0/TTS.Tool_0.1.0_amd64.AppImage) |

Todas las versiones: [página de Releases](https://github.com/blancofran/TTS-Tool-Src/releases).
Los nuevos builds se publican como **borradores** y solo se hacen públicos
cuando se revisan y publican manualmente.

## Cómo empezar

```bash
npm install
scripts/download-models.sh small   # o download-models.ps1 en Windows
npm run tauri dev
```

El modelo `small` debe existir en `src-tauri/resources/models/ggml-small-q5_1.bin`
antes de correr `dev` o `build` — es un recurso de Tauri incluido en el bundle,
no se descarga en tiempo de ejecución. Está en `.gitignore`; cada máquina de
desarrollo o corrida de CI lo descarga con el script de arriba.

## Estructura del proyecto

```
src/                        Frontend en React
  components/                DropZone, ModeSelector, ProgressBar, TranscriptView
  lib/tauri.ts                wrappers invoke()/listen() hacia comandos de Rust
  types.ts                    tipos TS compartidos que reflejan los DTOs de Rust

src-tauri/src/
  commands/                  puntos de entrada #[tauri::command] (transcribe, model, export)
  audio/ffmpeg.rs             entrada -> WAV mono 16kHz vía ffmpeg-sidecar
  whisper/engine.rs           WAV -> transcripción + segmentos con timestamps
  models/                     catálogo de modelos, resolución de rutas, descarga bajo demanda
  error.rs                    AppError, serializado al frontend como {kind, message}

scripts/                     download-models.sh / .ps1 (descarga modelos GGML + fija SHA-256)
```

## Notas / limitaciones conocidas (fase 1)

- El repositorio de HuggingFace de ggerganov/whisper.cpp no publica checksums
  oficiales para estos archivos. La integridad se verifica con "confianza en
  el primer uso": el SHA-256 se calcula justo después de la descarga y se
  guarda junto al modelo, y se vuelve a comprobar en corridas posteriores
  para detectar corrupción local (no para autenticar el archivo original).
- El idioma siempre se detecta automáticamente; todavía no hay selector de idioma.
- La exportación `.srt`/`.vtt` y el flujo de descarga del modelo medium ya
  están implementados en el backend de Rust pero aún no están conectados a
  una UX de "primera vez con precisión alta" (aviso de tamaño + diálogo de
  confirmación antes de descargar).

## Configuración recomendada de IDE

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
