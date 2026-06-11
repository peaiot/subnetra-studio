$ErrorActionPreference = 'Stop'

Write-Host '==========================================' -ForegroundColor Cyan
Write-Host ' Start building Subnetra Web (Linux Musl Static)...' -ForegroundColor Cyan
Write-Host '==========================================' -ForegroundColor Cyan

Write-Host '[1/3] Compiling with Cargo in WSL...' -ForegroundColor Yellow
wsl ~/.cargo/bin/cargo build --release --target x86_64-unknown-linux-musl --manifest-path web-server/Cargo.toml
if ($LASTEXITCODE -ne 0) {
    Write-Host ' Build failed. Please check the logs.' -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host '[2/3] Preparing output directory...' -ForegroundColor Yellow
$OutputDir = 'output/linux'
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
}

Write-Host '[3/3] Extracting artifacts...' -ForegroundColor Yellow
$SourceFile = 'target/x86_64-unknown-linux-musl/release/web-server'
$DestFile = "$OutputDir/subnetra-web"

Copy-Item $SourceFile -Destination $DestFile -Force

Write-Host '==========================================' -ForegroundColor Green
Write-Host ' Build successful!' -ForegroundColor Green
Write-Host " Artifact path: $DestFile" -ForegroundColor Green
Write-Host '==========================================' -ForegroundColor Green
