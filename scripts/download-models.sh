#!/usr/bin/env bash
# Downloads the GGML Whisper models used by TTS Tool from the official
# ggerganov/whisper.cpp HuggingFace repo, verifying integrity via a
# trust-on-first-use SHA-256 (the upstream repo does not publish checksums).
#
# Usage:
#   scripts/download-models.sh small     # bundled "Fast" model (~190MB)
#   scripts/download-models.sh medium    # on-demand "High accuracy" model (~539MB)
#   scripts/download-models.sh all
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MODELS_DIR="$REPO_ROOT/src-tauri/resources/models"
BASE_URL="https://huggingface.co/ggerganov/whisper.cpp/resolve/main"

mkdir -p "$MODELS_DIR"

download_model() {
  local file_name="$1"
  local dest="$MODELS_DIR/$file_name"

  if [ -f "$dest" ]; then
    echo "Already downloaded: $file_name"
    return 0
  fi

  echo "Downloading $file_name ..."
  curl -L --fail --retry 5 --retry-delay 5 -o "$dest.part" "$BASE_URL/$file_name"
  mv "$dest.part" "$dest"

  sha256sum "$dest" 2>/dev/null | awk '{print $1}' > "$dest.sha256" \
    || shasum -a 256 "$dest" | awk '{print $1}' > "$dest.sha256"
  echo "Saved $dest"
  echo "SHA-256: $(cat "$dest.sha256")"
}

case "${1:-}" in
  small)
    download_model "ggml-small-q5_1.bin"
    ;;
  medium)
    download_model "ggml-medium-q5_0.bin"
    ;;
  all)
    download_model "ggml-small-q5_1.bin"
    download_model "ggml-medium-q5_0.bin"
    ;;
  *)
    echo "Usage: $0 {small|medium|all}" >&2
    exit 1
    ;;
esac
