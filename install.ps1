# Odyn installer for Windows
# https://codeberg.org/razkar/odyn

$ErrorActionPreference = "Stop"

$Repo = "razkar/odyn"
$BinaryName = "odyn.exe"

function Write-Info    { Write-Host ("    install " + $args[0]) -ForegroundColor Blue }
function Write-Success { Write-Host ("    install " + $args[0]) -ForegroundColor Green }
function Write-Warn    { Write-Host ("       warn " + $args[0]) -ForegroundColor Yellow }
function Write-Fail    { Write-Host ("      error " + $args[0]) -ForegroundColor Red; throw "fatal" }

$arch = $env:PROCESSOR_ARCHITECTURE
$binary = switch ($arch) {
    "AMD64" { "odyn-windows-x86_64.exe" }
    "x86"   { "odyn-windows-i686.exe" }
    "ARM64" { Write-Fail "Windows ARM64 is not yet supported. check https://codeberg.org/razkar/odyn/releases for updates." }
    default { Write-Fail "unsupported architecture: $arch" }
}

# Detect admin
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
    [Security.Principal.WindowsBuiltInRole]::Administrator
)

# Choose install location
Write-Host ""
Write-Host "  Install type:" -ForegroundColor Cyan
Write-Host ("    1) User        " + $env:USERPROFILE + "\.local\bin")
Write-Host  "    2) System-wide $env:ProgramFiles\odyn  (requires Administrator)"
$installChoice = Read-Host "  Choice [1]"
Write-Host ""

if ($installChoice -eq "2") {
    if (-not $isAdmin) {
        Write-Fail "system-wide install requires Administrator. re-run this script as Administrator."
    }
    $InstallDir = "$env:ProgramFiles\odyn"
    $PathScope = "Machine"
} else {
    $InstallDir = "$env:USERPROFILE\.local\bin"
    $PathScope = "User"
}

Write-Info "fetching latest version..."
try {
    $api = Invoke-RestMethod "https://codeberg.org/api/v1/repos/$Repo/releases/latest"
    $version = $api.tag_name
} catch {
    Write-Fail "could not fetch latest version from Codeberg API: $_"
}

if (-not $version) {
    Write-Fail "could not determine latest version"
}

Write-Info "latest version is $version"

$baseUrl = "https://codeberg.org/$Repo/releases/download/$version"
$binaryUrl = "$baseUrl/$binary"
$sumsUrl = "$baseUrl/SHA256SUMS"

$tmpDir = [System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid().ToString()
New-Item -ItemType Directory -Path $tmpDir | Out-Null
$tmpBinary = "$tmpDir\$binary"

try {
    Write-Info "downloading $binary..."
    try {
        Invoke-WebRequest -Uri $binaryUrl -OutFile $tmpBinary -UseBasicParsing
    } catch {
        Write-Fail "failed to download binary: $_"
    }

    if ((Get-Item $tmpBinary).Length -eq 0) {
        Write-Fail "downloaded file is empty"
    }

    Write-Info "verifying checksum..."
    $expected = $null
    try {
        $sumsContent = Invoke-RestMethod $sumsUrl
        $matchingLine = ($sumsContent -split "`r?`n" | Where-Object { $_ -match " $([regex]::Escape($binary))$" } | Select-Object -First 1)
        if ($matchingLine) {
            $expected = ($matchingLine -split "\s+" | Where-Object { $_ -ne "" } | Select-Object -First 1).Trim()
        }
    } catch {
        Write-Warn "could not fetch SHA256SUMS, skipping verification"
        $expected = $null
    }

    if ($expected) {
        $actual = (Get-FileHash $tmpBinary -Algorithm SHA256).Hash.ToLower()
        $expectedNorm = $expected.ToLower().Trim()
        if ($actual -ne $expectedNorm) {
            Write-Fail "SHA256 mismatch! expected $expectedNorm, got $actual. aborting."
        }
        Write-Success "checksum verified"
    } else {
        Write-Warn "skipped checksum verification"
    }

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir | Out-Null
    }

    Move-Item -Force $tmpBinary "$InstallDir\$BinaryName"
} finally {
    if (Test-Path $tmpDir) {
        Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue
    }
}

Write-Success "odyn $version installed to $InstallDir\$BinaryName"

$currentPath = [Environment]::GetEnvironmentVariable("PATH", $PathScope)
$pathEntries = if ($currentPath) { $currentPath -split ";" } else { @() }
if ($pathEntries -contains $InstallDir) {
    Write-Success "$InstallDir is already on your PATH. you're good to go!"
} else {
    Write-Warn "$InstallDir is not on your PATH."
    Write-Warn "adding it now..."
    $newPath = if ($currentPath) { "$currentPath;$InstallDir" } else { $InstallDir }
    [Environment]::SetEnvironmentVariable("PATH", $newPath, $PathScope)
    Write-Success "PATH updated. restart your terminal for changes to take effect."
}
