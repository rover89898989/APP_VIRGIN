# ==============================================================================
# TYPE SYNC SCRIPT
# ==============================================================================
#
# Generates TypeScript types from Rust and copies them to the mobile app.
#
# USAGE: .\scripts\sync-types.ps1
#
# ==============================================================================

$ErrorActionPreference = "Stop"

$ROOT_DIR = Split-Path -Parent $PSScriptRoot
$BACKEND_DIR = Join-Path $ROOT_DIR "backend"
$BINDINGS_DIR = Join-Path $BACKEND_DIR "bindings"
$MOBILE_TYPES_DIR = Join-Path $ROOT_DIR "mobile\src\api\types"

Write-Host "=== TypeScript Type Generation ===" -ForegroundColor Cyan

# Step 1: Generate types by running cargo test
Write-Host "`n[1/3] Generating types from Rust..." -ForegroundColor Yellow
Push-Location $BACKEND_DIR
try {
    cargo test generate_typescript_types --quiet 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Warning: Type generation test had issues, but continuing..." -ForegroundColor Yellow
    }
} finally {
    Pop-Location
}

# Step 2: Check if bindings were generated
Write-Host "[2/3] Checking generated bindings..." -ForegroundColor Yellow
if (-not (Test-Path $BINDINGS_DIR)) {
    Write-Host "No bindings directory found. Types may not have been generated." -ForegroundColor Yellow
    Write-Host "This is expected on first run. Types are defined in mobile/src/api/types/" -ForegroundColor Yellow
    exit 0
}

$typeFiles = Get-ChildItem -Path $BINDINGS_DIR -Filter "*.ts" -ErrorAction SilentlyContinue
if ($null -eq $typeFiles -or $typeFiles.Count -eq 0) {
    Write-Host "No .ts files found in bindings directory." -ForegroundColor Yellow
    exit 0
}

# Step 3: Copy types to mobile app
Write-Host "[3/3] Copying types to mobile app..." -ForegroundColor Yellow

# Ensure mobile types directory exists
if (-not (Test-Path $MOBILE_TYPES_DIR)) {
    New-Item -ItemType Directory -Path $MOBILE_TYPES_DIR -Force | Out-Null
}

foreach ($file in $typeFiles) {
    $destPath = Join-Path $MOBILE_TYPES_DIR $file.Name
    Copy-Item -Path $file.FullName -Destination $destPath -Force
    Write-Host "  Copied: $($file.Name)" -ForegroundColor Green
}

Write-Host "`n=== Type sync complete! ===" -ForegroundColor Cyan
Write-Host "Types are now in: $MOBILE_TYPES_DIR" -ForegroundColor Gray
