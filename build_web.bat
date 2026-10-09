@echo off
echo ====================================================
echo   Building Die by Glory for Web (itch.io / WASM)
echo ====================================================
echo.
cargo build --target wasm32-unknown-unknown --release
if %errorlevel% neq 0 (
    echo [ERROR] WebAssembly compilation failed!
    pause
    exit /b %errorlevel%
)

if not exist dist_web mkdir dist_web
copy /y target\wasm32-unknown-unknown\release\die_by_glory.wasm dist_web\die_by_glory.wasm >nul

echo.
echo Packaging dist_web.zip for itch.io upload...
powershell -Command "Compress-Archive -Path dist_web\* -DestinationPath dist_web.zip -Force"

echo.
echo ====================================================
echo [SUCCESS] Web build complete!
echo   - Folder: dist_web\ (index.html, mq_js_bundle.js, die_by_glory.wasm)
echo   - Itch.io ready file: dist_web.zip
echo ====================================================
echo.
