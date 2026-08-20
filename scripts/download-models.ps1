#!/usr/bin/env pwsh
# Downloads the GGML Whisper models used by TTS Tool from the official
# ggerganov/whisper.cpp HuggingFace repo, verifying integrity via a
# trust-on-first-use SHA-256 (the upstream repo does not publish checksums).
#
# Usage:
#   scripts/download-models.ps1 small     # bundled "Fast" model (~190MB)
#   scripts/download-models.ps1 medium    # on-demand "High accuracy" model (~539MB)
#   scripts/download-models.ps1 all
param(
    [Parameter(Mandatory = $true)]
    [ValidateSet("small", "medium", "all")]
    [string]$Target
)

$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$ModelsDir = Join-Path $RepoRoot "src-tauri/resources/models"
$BaseUrl = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main"

New-Item -ItemType Directory -Force -Path $ModelsDir | Out-Null

function Get-Model {
    param([string]$FileName)

    $dest = Join-Path $ModelsDir $FileName
    if (Test-Path $dest) {
        Write-Host "Already downloaded: $FileName"
        return
    }

    Write-Host "Downloading $FileName ..."
    $partial = "$dest.part"
    Invoke-WebRequest -Uri "$BaseUrl/$FileName" -OutFile $partial
    Move-Item -Force $partial $dest

    $hash = (Get-FileHash -Algorithm SHA256 -Path $dest).Hash.ToLower()
    Set-Content -Path "$dest.sha256" -Value $hash -NoNewline
    Write-Host "Saved $dest"
    Write-Host "SHA-256: $hash"
}

switch ($Target) {
    "small" { Get-Model "ggml-small-q5_1.bin" }
    "medium" { Get-Model "ggml-medium-q5_0.bin" }
    "all" {
        Get-Model "ggml-small-q5_1.bin"
        Get-Model "ggml-medium-q5_0.bin"
    }
}
