# 设置控制台编码为 UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

# 设置工作目录为脚本所在项目根目录
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Split-Path -Parent $scriptPath)

# Clean src-setup\targetdir
if (Test-Path "src-setup\targetdir") {
    Remove-Item "src-setup\targetdir" -Recurse -Force
    Write-Host "Cleaned src-setup\targetdir"
}

# Clean src-setup\output
if (Test-Path "src-setup\output") {
    Remove-Item "src-setup\output" -Recurse -Force
    Write-Host "Cleaned src-setup\output"
}

Write-Host "Clean completed"
