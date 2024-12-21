<#
.SYNOPSIS
    Build script for ShellMorph Pro (Windows).

.DESCRIPTION
    This script compiles all components of the ShellMorph Pro project:
    - Rust core
    - CLI
    - API

.PARAMETER Clean
    Removes all build artifacts before building.

.PARAMETER BuildType
    Specifies the build type (Release/Debug). Default is Release.

.EXAMPLE
    .\build.ps1 -BuildType Debug
#>

param(
    [switch]$Clean,
    [string]$BuildType = "Release"
)

# Define paths
$CoreDir = "src/core"
$CliDir = "src/cli"
$ApiDir = "src/api"
$OutputDir = "dist"

# Clean build artifacts
if ($Clean) {
    Write-Host "Cleaning build directories..." -ForegroundColor Yellow
    Remove-Item -Recurse -Force "$OutputDir", "$CoreDir\target", "$CliDir\build", "$ApiDir\build"
    Write-Host "Clean complete." -ForegroundColor Green
    exit
}

# Step 1: Build the Rust core
Write-Host "Building Rust core..." -ForegroundColor Yellow
Start-Process cargo -ArgumentList "build --release --manifest-path $CoreDir\Cargo.toml" -Wait

# Copy output
New-Item -ItemType Directory -Force -Path "$OutputDir\core"
Copy-Item "$CoreDir\target\release\libshellmorph_core.a" "$OutputDir\core"

# Step 2: Build the CLI
Write-Host "Building CLI..." -ForegroundColor Yellow
cmake -S $CliDir -B "$CliDir\build" -DCMAKE_BUILD_TYPE=$BuildType
cmake --build "$CliDir\build" --target shellmorph-cli

# Copy output
New-Item -ItemType Directory -Force -Path "$OutputDir\cli"
Copy-Item "$CliDir\build\shellmorph-cli.exe" "$OutputDir\cli"

# Step 3: Build the API
Write-Host "Building API..." -ForegroundColor Yellow
Start-Process npm -ArgumentList "install --prefix $ApiDir" -Wait
Start-Process npm -ArgumentList "run build --prefix $ApiDir" -Wait

# Copy output
New-Item -ItemType Directory -Force -Path "$OutputDir\api"
Copy-Item -Recurse -Force "$ApiDir\build\*" "$OutputDir\api"

Write-Host "Build completed successfully!" -ForegroundColor Green
Write-Host "Output directory: $OutputDir"
