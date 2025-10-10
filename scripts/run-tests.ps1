#!/usr/bin/env pwsh
# FerrisScript Test Runner
# Convenience script for running test harness examples

param(
    [Parameter(Mandatory = $false)]
    [string]$Script = "",
    
    [Parameter(Mandatory = $false)]
    [switch]$All,
    
    [Parameter(Mandatory = $false)]
    [switch]$Fast,
    
    [Parameter(Mandatory = $false)]
    [switch]$Verbose,
    
    [Parameter(Mandatory = $false)]
    [string]$Filter = ""
)

# Color output functions
function Write-Success {
    param([string]$Message)
    Write-Host "✅ $Message" -ForegroundColor Green
}

function Write-Error-Msg {
    param([string]$Message)
    Write-Host "❌ $Message" -ForegroundColor Red
}

function Write-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor Cyan
}

function Write-Warning-Msg {
    param([string]$Message)
    Write-Host "⚠️  $Message" -ForegroundColor Yellow
}

# Get project root
$ProjectRoot = Split-Path -Parent $PSScriptRoot
Set-Location $ProjectRoot

# Build the test harness in release mode if not in fast mode
if (-not $Fast) {
    Write-Info "Building test harness in release mode..."
    cargo build --release -p ferrisscript_test_harness
    if ($LASTEXITCODE -ne 0) {
        Write-Error-Msg "Build failed!"
        exit 1
    }
    Write-Success "Build complete"
    Write-Host ""
}

# Build the command
$TestCmd = "cargo run --release --bin ferris-test --"

# Add script argument if provided
if ($Script -ne "") {
    $TestCmd += " --script $Script"
}

# Add --all flag if requested
if ($All) {
    $TestCmd += " --all"
}

# Add filter if provided
if ($Filter -ne "") {
    $TestCmd += " --filter $Filter"
}

# Add verbose flag if requested
if ($Verbose) {
    $TestCmd += " --verbose"
}

# Display what we're running
Write-Info "Running: $TestCmd"
Write-Host ""

# Execute the command
Invoke-Expression $TestCmd

# Check exit code
if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Success "All tests passed!"
    exit 0
} else {
    Write-Host ""
    Write-Error-Msg "Tests failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}
