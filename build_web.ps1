Write-Host "====================================================" -ForegroundColor Cyan
Write-Host "  Building Die by Glory for Web (itch.io / WASM)    " -ForegroundColor Cyan
Write-Host "====================================================" -ForegroundColor Cyan

cargo build --target wasm32-unknown-unknown --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Cargo WebAssembly build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

if (-not (Test-Path "dist_web")) {
    New-Item -ItemType Directory -Path "dist_web" | Out-Null
}

Copy-Item "target\wasm32-unknown-unknown\release\die_by_glory.wasm" -Destination "dist_web\die_by_glory.wasm" -Force

Write-Host "`nPackaging dist_web.zip for itch.io..." -ForegroundColor Yellow
Compress-Archive -Path "dist_web\*" -DestinationPath "dist_web.zip" -Force

Write-Host "`n====================================================" -ForegroundColor Green
Write-Host "[SUCCESS] Web build complete!" -ForegroundColor Green
Write-Host "  - Deployment folder: dist_web\" -ForegroundColor White
Write-Host "  - Itch.io ready file: dist_web.zip" -ForegroundColor White
Write-Host "====================================================" -ForegroundColor Green
