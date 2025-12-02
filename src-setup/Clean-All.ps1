# 设置控制台编码为 UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

# 设置工作目录为脚本所在项目根目录
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Split-Path -Parent $scriptPath)

# Clean npm dist
Write-Host "Cleaning npm dist..."
if (Test-Path "dist") {
    Remove-Item "dist" -Recurse -Force
}

# Clean setup directories
Write-Host "Cleaning setup directories..."
if (Test-Path "src-setup\targetdir") {
    Remove-Item "src-setup\targetdir" -Recurse -Force
}
if (Test-Path "src-setup\output") {
    Remove-Item "src-setup\output" -Recurse -Force
}

# Run Rust clean
Write-Host "Running Rust clean..."
Set-Location "src-tauri"
cargo clean
Set-Location ..

Write-Host "All clean completed"
