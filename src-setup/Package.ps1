# 设置控制台编码为 UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

# 设置工作目录为脚本所在项目根目录
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Split-Path -Parent $scriptPath)

# Clean previous build artifacts
Write-Host "Cleaning previous build artifacts..."
if (Test-Path "src-setup\targetdir") {
    Remove-Item "src-setup\targetdir" -Recurse -Force
}
if (Test-Path "src-setup\output") {
    Remove-Item "src-setup\output" -Recurse -Force
}

# Build Tauri application
Write-Host "Building Tauri application..."
npm run tauri build
if ($LASTEXITCODE -ne 0) {
    Write-Error "Tauri build failed with exit code: $LASTEXITCODE"
    exit $LASTEXITCODE
}

# 创建目标目录
$targetDir = "src-setup\targetdir"
if (-not (Test-Path $targetDir)) {
    New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
}

# 复制 StarRailAssistant.exe 和 StarRailAssistant.pdb
$releaseDir = "src-tauri\target\release"
Copy-Item "$releaseDir\StarRailAssistant.exe" -Destination $targetDir -Force
Copy-Item "$releaseDir\StarRailAssistant.pdb" -Destination $targetDir -Force

# 复制 StarRailAssistant 文件夹内容
Copy-Item "StarRailAssistant\*" -Destination $targetDir -Recurse -Force

# Check for Inno Setup compiler
try {
    $isccPath = Get-Command iscc.exe -ErrorAction Stop
    Write-Host "Found Inno Setup compiler: $($isccPath.Source)"
} catch {
    Write-Error "ERROR: Inno Setup compiler not found. Please install Inno Setup and add it to PATH."
    Write-Host "Recommended: choco install innosetup -y"
    exit 1
}

# Get version from package.json
$packageJson = Get-Content "package.json" -Raw | ConvertFrom-Json
$version = $packageJson.version
Write-Host "Detected version: $version"

# Update version in setup.iss
$setupIssPath = "src-setup\setup.iss"
$setupIssContent = Get-Content $setupIssPath -Raw
$setupIssContent = $setupIssContent -replace '#define MyAppVersion "unknown"', "#define MyAppVersion `"$version`""
Set-Content $setupIssPath -Value $setupIssContent -NoNewline

# Create output directory
$outputDir = "src-setup\output"
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

# Create zip archive
Write-Host "Creating zip archive..."
$zipFileName = "StarRailAssistant-$version-portable.zip"
$zipPath = Join-Path $outputDir $zipFileName
Compress-Archive -Path "$targetDir\*" -DestinationPath $zipPath -Force
Write-Host "Zip archive created: $zipFileName"

# Compile with Inno Setup
Write-Host "Building installer..."
iscc "src-setup\setup.iss"

# Restore version to unknown
$setupIssContent = $setupIssContent -replace "#define MyAppVersion `"$version`"", '#define MyAppVersion "unknown"'
Set-Content $setupIssPath -Value $setupIssContent -NoNewline

if ($LASTEXITCODE -eq 0) {
    Write-Host "Done! Installer created in src-setup\output\"
} else {
    Write-Error "Build failed with exit code: $LASTEXITCODE"
    exit $LASTEXITCODE
}
