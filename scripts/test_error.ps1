#!/usr/bin/env pwsh
# Test FerrisScript Error Messages
# 
# This script allows you to test FerrisScript files and see error messages
# with source context display.
#
# Usage: .\test_error.ps1 <path-to-ferris-file>
# Example: .\test_error.ps1 examples\error_showcase.ferris

param(
    [Parameter(Mandatory=$true)]
    [string]$FilePath
)

Write-Host "Testing FerrisScript file: $FilePath" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

if (-not (Test-Path $FilePath)) {
    Write-Host "Error: File not found: $FilePath" -ForegroundColor Red
    exit 1
}

# Read the file content
$content = Get-Content $FilePath -Raw

# Create a temporary test file
$testContent = @"
#[test]
fn test_error_showcase() {
    let source = r#"$content"#;
    let result = ferrisscript_compiler::compile(source);
    
    match result {
        Ok(_) => {
            println!("\n✓ Compilation successful!");
            println!("The script compiled without errors.\n");
        }
        Err(e) => {
            println!("\n✗ Compilation failed with error:");
            println!("{}\n", e);
            panic!("Compilation failed (this shows the error message above)");
        }
    }
}
"@

$testFile = Join-Path $env:TEMP "ferrisscript_test_$(Get-Random).rs"
Set-Content -Path $testFile -Value $testContent

Write-Host "Running compiler test..." -ForegroundColor Yellow
Write-Host ""

# Run the test (this will show the error message if there is one)
cargo test --package ferrisscript_compiler --test error_showcase 2>&1 | Write-Host

Write-Host ""
Write-Host "=" * 60 -ForegroundColor Cyan

# Cleanup
Remove-Item $testFile -ErrorAction SilentlyContinue
