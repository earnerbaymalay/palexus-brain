# Quick Start — Build Installers

## Prerequisites
- Rust 1.75+ (via rustup)
- Node.js 20+ and pnpm
- System libraries (Linux):
  ```bash
  sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libssl-dev libdbus-1-dev libjavascriptcoregtk-4.1-dev libsoup-3.0-dev
  ```
- macOS: Xcode Command Line Tools
- Windows: Visual Studio Build Tools + WebView2

## Build All Artifacts
```bash
# 1. Build the CLI binary
cargo build -p palexus --release
# Binary at: target/release/palexus

# 2. Build the desktop frontend
cd apps/desktop
pnpm install
pnpm build
# Frontend dist at: apps/desktop/dist/

# 3. Generate icons (run once)
cd src-tauri
python3 build-icons.py

# 4. Build the Tauri desktop app + installers
cargo build --release
pnpm tauri build
# Installers at: apps/desktop/src-tauri/target/release/bundle/
```

## Platform Installers
- **Linux**
  - Format: .deb
  - Location: `.../bundle/deb/palexus_0.7.4_amd64.deb`
- **Linux**
  - Format: .AppImage
  - Location: `.../bundle/appimage/palexus_0.7.4_amd64.AppImage`
- **macOS**
  - Format: .dmg
  - Location: `.../bundle/dmg/palexus_0.7.4_x64.dmg` (requires Apple Developer ID)
- **Windows**
  - Format: .msi
  - Location: `.../bundle/msi/palexus_0.7.4_x64_en-US.msi` (requires code signing cert)

## CLI Only (no desktop)
If you only need the CLI, you can skip the Tauri build entirely:
```bash
cargo build -p palexus --release
sudo cp target/release/palexus /usr/local/bin/
```

---

# Palexus — Release Packaging & Distribution Guide

This document specifies the release packaging configurations, installer scripts, platform bundlers, code signing workflows, and package manager formulae for **Palexus** (Desktop GUI and CLI engine).

---

## 1. Overview & Distribution Strategy

Palexus is distributed in two primary forms:
1. **Desktop GUI Application**: A Tauri v2 application shipping native installers for macOS (`.dmg`), Windows (`.msi`), and Linux (`.AppImage`, `.deb`).
2. **CLI Engine (`palexus`)**: A standalone, zero-dependency Rust binary distributed via Homebrew, shell installer scripts (`install.sh`, `install.ps1`), and direct release tarballs/zip archives.

---

## 2. Tauri v2 Bundle Configuration

Below is the production `bundle` configuration block for `apps/desktop/src-tauri/tauri.conf.json` targeting Tauri v2:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Palexus",
  "version": "0.7.4",
  "identifier": "dev.palexus.desktop",
  "build": {
    "frontendDist": "../dist"
  },
  "app": {
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:;"
    },
    "windows": [
      {
        "title": "Palexus",
        "width": 1280,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ]
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "msi", "appimage", "deb"],
    "identifier": "dev.palexus.desktop",
    "publisher": "Palexus Core Team",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "resources": [],
    "copyright": "Copyright © 2026 Palexus Contributors",
    "category": "DeveloperTool",
    "shortDescription": "Universal agent harness and multi-tool config migration engine",
    "longDescription": "Palexus provides deterministic sync, conflict resolution, security sandboxing, and drift detection across 12 AI developer tools.",
    "macOS": {
      "frameworks": [],
      "minimumSystemVersion": "10.15.0",
      "exceptionDomain": "",
      "signing": {
        "identity": "Developer ID Application: Palexus LLC (XYZ1234567)"
      },
      "entitlements": "entitlements.plist",
      "dmg": {
        "background": "assets/dmg_background.png",
        "windowSize": {
          "width": 660,
          "height": 400
        },
        "appPosition": {
          "x": 180,
          "y": 170
        },
        "applicationFolderPosition": {
          "x": 480,
          "y": 170
        }
      }
    },
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com",
      "wix": {
        "upgradeCode": "e89c6f2a-5b12-4d39-9f1e-7b89c0d1e2f3",
        "language": ["en-US"],
        "bannerPath": "assets/wix_banner.bmp",
        "dialogImagePath": "assets/wix_dialog.bmp"
      }
    },
    "linux": {
      "deb": {
        "depends": [
          "libc6 (>= 2.31)",
          "libgtk-3-0 (>= 3.24.0)",
          "libwebkit2gtk-4.1-0 (>= 2.38.0)",
          "libayatana-appindicator3-1"
        ],
        "desktopTemplate": "assets/palexus.desktop",
        "section": "devel",
        "priority": "optional"
      },
      "appimage": {
        "bundleMediaFramework": false
      }
    }
  }
}
```

---

## 3. Platform Installer Specifications

### 3.1 macOS Package (.dmg) & Notarization
- **Artifact**: `Palexus_0.7.4_x64.dmg`, `Palexus_0.7.4_aarch64.dmg`
- **Signing**: Signed via `codesign` using Apple Developer ID Application certificate.
- **Notarization Workflow**:
  ```bash
  # Submit DMG for Apple Notarization
  xcrun notarytool submit target/release/bundle/dmg/palexus_0.7.4_aarch64.dmg \
    --apple-id "$APPLE_ID" \
    --team-id "$APPLE_TEAM_ID" \
    --password "$APPLE_APP_SPECIFIC_PASSWORD" \
    --wait

  # Staple ticket to DMG
  xcrun stapler staple target/release/bundle/dmg/palexus_0.7.4_aarch64.dmg
  ```

### 3.2 Windows Package (.msi) & Authenticode
- **Artifact**: `Palexus_0.7.4_x64_en-US.msi`
- **Engine**: WiX Toolset v3/v4 wrapped by Tauri CLI.
- **Signing**: Authenticode signing via Azure Key Vault / Signtool:
  ```powershell
  signtool.exe sign `
    /tr http://timestamp.digicert.com `
    /td sha256 `
    /fd sha256 `
    /sha1 "$CODE_SIGNING_CERT_THUMBPRINT" `
    target/release/bundle/msi/palexus_0.7.4_x64_en-US.msi
  ```

### 3.3 Linux Debian Package (.deb)
- **Artifact**: `Palexus_0.7.4_amd64.deb`
- **Dependencies**: `libc6`, `libgtk-3-0`, `libwebkit2gtk-4.1-0`, `libayatana-appindicator3-1`.
- **Maintainer Scripts**:
  - `postinst`: Registers MIME types and updates desktop database (`update-desktop-database -q`).
  - `postrm`: Deletes desktop caches on removal.

### 3.4 Linux AppImage
- **Artifact**: `Palexus_0.7.4_amd64.AppImage`
- **Portable Design**: Self-contained runtime bundling standard webkit dependencies, using `AppRun` script launcher.

---

## 4. Universal Shell Installer Script (`install.sh`)

POSIX-compliant shell installer script for CLI deployment on Linux and macOS:

```bash
#!/bin/sh
# Palexus CLI Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/earnerbaymalay/palexus/main/scripts/install.sh | sh

set -e

REPO="earnerbaymalay/palexus"
VERSION="${VERSION:-latest}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

detect_platform() {
  OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
  ARCH="$(uname -m)"

  case "$OS" in
    linux) OS="unknown-linux-gnu" ;;
    darwin) OS="apple-darwin" ;;
    *) echo "Error: Unsupported OS $OS"; exit 1 ;;
  esac

  case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="aarch64" ;;
    *) echo "Error: Unsupported architecture $ARCH"; exit 1 ;;
  esac

  TARGET="${ARCH}-${OS}"
}

fetch_release() {
  detect_platform
  echo "Installing Palexus CLI ($TARGET)..."

  if [ "$VERSION" = "latest" ]; stream
    DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/palexus-${TARGET}.tar.gz"
    CHECKSUM_URL="https://github.com/${REPO}/releases/latest/download/SHA256SUMS"
  else
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/v${VERSION}/palexus-${TARGET}.tar.gz"
    CHECKSUM_URL="https://github.com/${REPO}/releases/download/v${VERSION}/SHA256SUMS"
  fi

  TMP_DIR="$(mktemp -d)"
  trap 'rm -rf "$TMP_DIR"' EXIT

  echo "Downloading $DOWNLOAD_URL..."
  curl -sSL "$DOWNLOAD_URL" \
    -o "$TMP_DIR/palexus.tar.gz"
  curl -sSL "$CHECKSUM_URL" \
    -o "$TMP_DIR/SHA256SUMS"

  echo "Verifying SHA-256 checksum..."
  (cd "$TMP_DIR" && \
    grep "palexus-${TARGET}.tar.gz" SHA256SUMS | \
    sha256sum -c -)

  echo "Extracting binary..."
  tar -xzf "$TMP_DIR/palexus.tar.gz" -C "$TMP_DIR"

  echo "Installing to $INSTALL_DIR..."
  if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_DIR/palexus" "$INSTALL_DIR/palexus"
  else
    sudo mv "$TMP_DIR/palexus" "$INSTALL_DIR/palexus"
  fi

  chmod +x "$INSTALL_DIR/palexus"
  echo "Successfully installed palexus to $INSTALL_DIR/palexus"
  "$INSTALL_DIR/palexus" --version
}

fetch_release
```

---

## 5. Windows PowerShell Installer Script (`install.ps1`)

PowerShell script for automated Windows CLI installation:

```powershell
# Palexus CLI Windows Installer
# Usage: iwr -useb https://raw.githubusercontent.com/earnerbaymalay/palexus/main/scripts/install.ps1 | iex

$ErrorActionPreference = 'Stop'

$Repo = "earnerbaymalay/palexus"
$InstallDir = "$env:LOCALAPPDATA\Programs\Palexus\bin"

function Install-Palexus {
    $Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { throw "32-bit Windows is not supported." }
    $Target = "$Arch-pc-windows-msvc"
    
    $DownloadUrl = "https://github.com/$Repo/releases/latest/download/palexus-$Target.zip"
    $ChecksumUrl = "https://github.com/$Repo/releases/latest/download/SHA256SUMS"

    $TmpDir = New-Item -ItemType Directory -Path ([System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid().ToString())
    try {
        Write-Host "Downloading Palexus CLI ($Target)..." -ForegroundColor Cyan
        Invoke-WebRequest -Uri $DownloadUrl -OutFile "$TmpDir\palexus.zip"
        Invoke-WebRequest -Uri $ChecksumUrl -OutFile "$TmpDir\SHA256SUMS"

        Write-Host "Verifying checksum..." -ForegroundColor Cyan
        $ExpectedHash = (Get-Content "$TmpDir\SHA256SUMS" | Select-String "palexus-$Target.zip").Line.Split(" ")[0]
        $ActualHash = (Get-FileHash "$TmpDir\palexus.zip" -Algorithm SHA256).Hash.ToLower()

        if ($ExpectedHash -ne $ActualHash) {
            throw "Checksum verification failed! Expected: $ExpectedHash, Got: $ActualHash"
        }

        Write-Host "Extracting binary..." -ForegroundColor Cyan
        Expand-Archive -Path "$TmpDir\palexus.zip" -DestinationPath $TmpDir -Force

        if (!(Test-Path $InstallDir)) {
            New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
        }

        Move-Item -Path "$TmpDir\palexus.exe" -Destination "$InstallDir\palexus.exe" -Force

        # User PATH updates
        $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($UserPath -notlike "*$InstallDir*") {
            [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
            Write-Host "Added $InstallDir to User PATH." -ForegroundColor Green
        }

        Write-Host "Palexus CLI successfully installed to $InstallDir\palexus.exe" -ForegroundColor Green
    }
    finally {
        Remove-Item -Recurse -Force $TmpDir
    }
}

Install-Palexus
```

---

## 6. Homebrew Formula Draft (`palexus.rb`)

Formula draft for Homebrew deployment (`brew install palexus`):

```ruby
class Palexus < Formula
  desc "Universal agent harness & multi-tool config migration engine"
  homepage "https://github.com/earnerbaymalay/palexus-brain"
  url "https://github.com/earnerbaymalay/palexus-brain/releases/download/v0.7.4/palexus-src-v0.7.4.tar.gz"
  sha256 "a1b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0"
  license "MIT"
  head "https://github.com/earnerbaymalay/palexus-brain.git", branch: "main"

  bottle do
    root_url "https://github.com/earnerbaymalay/palexus-brain/releases/download/v0.7.4"
    sha256 cellar: :any_skip_relocation, arm64_sequoia: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    sha256 cellar: :any_skip_relocation, ventura:       "123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0"
    sha256 cellar: :any_skip_relocation, x86_64_linux:  "23456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef01"
  end

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: "apps/cli")
  end

  test do
    assert_match "palexus #{version}", shell_output("#{bin}/palexus --version")
    
    # Smoke test scan command on empty directory
    test_dir = testpath/"test_project"
    test_dir.mkpath
    output = shell_output("#{bin}/palexus scan --json", 0)
    assert_match "detected_tools", output
  end
end
```

---

## 7. Build Verification Checklist

Before publishing any release package:
- [ ] All `.dmg`, `.msi`, `.deb`, and `.AppImage` bundles pass automated smoke tests.
- [ ] Codesign signatures on `.dmg` and `.msi` are validated via `spctl` and `signtool`.
- [ ] `SHA256SUMS` file signed with release PGP / minisign key.
- [ ] Homebrew formula passes `brew audit --strict --online palexus.rb`.
- [ ] Install scripts tested on fresh macOS (ARM64/x86_64), Ubuntu 22.04/24.04, and Windows 11.
