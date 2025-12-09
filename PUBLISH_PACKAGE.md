# GitHub Package erstellen - SSL v7.0

## Was ist ein GitHub Package?

GitHub Packages ist eine Registry für Software-Pakete, ähnlich wie npm, PyPI, oder Docker Hub. User können dein Package direkt von GitHub installieren.

## Setup

### 1. Package-Konfiguration erstellen

Bereits erledigt:
- ✅ `.github/workflows/publish-package.yml` - Auto-Publish bei Tag
- ✅ `package.json` - Package Metadata

### 2. Personal Access Token (PAT) erstellen (falls nötig)

1. Gehe zu: https://github.com/settings/tokens
2. "Generate new token" → "Generate new token (classic)"
3. Name: "SSL Package Publishing"
4. Scopes auswählen:
   - ✅ `write:packages`
   - ✅ `read:packages`  
   - ✅ `delete:packages`
5. "Generate token"
6. Token kopieren (nur einmal sichtbar!)

### 3. Package manuell veröffentlichen

#### Option A: Via GitHub Actions (Automatisch)

```bash
# Einfach einen Tag pushen
git tag -a v7.0.0 -m "SSL v7.0.0 Release"
git push origin v7.0.0

# GitHub Actions erstellt automatisch das Package!
```

#### Option B: Via GitHub CLI (Manuell)

```bash
# GitHub CLI installieren
winget install GitHub.cli

# Login
gh auth login

# Package erstellen und hochladen
gh release create v7.0.0 release/ssl-windows-x64.zip `
  --title "SSL v7.0.0 - Native Compilation Edition" `
  --notes "Native x64 compilation, 9 languages NLP, Non-rectangular windows"

# Package zur Registry hinzufügen
gh api -X PUT /repos/SonnerStudio/Sonner-Studio-Language_SSL/packages/generic/ssl/7.0.0 `
  -F package=@release/ssl-windows-x64.zip
```

#### Option C: Via REST API

```powershell
# Package hochladen
$version = "7.0.0"
$token = "YOUR_GITHUB_TOKEN"
$repo = "SonnerStudio/Sonner-Studio-Language_SSL"

# Upload Binary
$headers = @{
    "Authorization" = "token $token"
    "Accept" = "application/vnd.github.v3+json"
}

$body = Get-Content release\ssl-windows-x64.zip -Raw -Encoding Byte

Invoke-RestMethod `
    -Uri "https://uploads.github.com/repos/$repo/packages/generic/ssl/$version/ssl-windows-x64.zip" `
    -Method PUT `
    -Headers $headers `
    -Body $body `
    -ContentType "application/zip"
```

### 4. Package verifizieren

Nach dem Upload sollte das Package hier erscheinen:
- Repository → Packages Tab
- Oder: https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/packages

## Installation für User

Sobald veröffentlicht, können User installieren:

### Via GitHub CLI

```bash
gh release download v7.0.0 --pattern "ssl-windows-x64.zip" --repo SonnerStudio/Sonner-Studio-Language_SSL
```

### Via Direct Download

```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-windows-x64.zip" -OutFile "ssl.zip"

# Extract
Expand-Archive ssl.zip -DestinationPath "$env:LOCALAPPDATA\SSL"
```

### Via Package Registry (Advanced)

Für fortgeschrittene User mit GitHub CLI:

```bash
# Authenticate
echo $GITHUB_TOKEN | gh auth login --with-token

# List packages
gh api repos/SonnerStudio/Sonner-Studio-Language_SSL/packages

# Download specific version
gh api /repos/SonnerStudio/Sonner-Studio-Language_SSL/packages/generic/ssl/versions/7.0.0 --jq '.html_url'
```

## Package-Features

**Was macht ein GitHub Package besser als nur ein Release?**

1. **Versionierung:** Automatische Version-Tracking
2. **Metriken:** Download-Statistiken
3. **Discovery:** Erscheint in GitHub Package Search
4. **Integration:** CI/CD Pipelines können direkt zugreifen
5. **Dependencies:** Kann Dependencies definieren

## Aktueller Status

✅ Package-Konfiguration erstellt
✅ GitHub Actions Workflow bereit
✅ Binary gebaut (8.69 MB)
⏳ Package veröffentlichen

**Nächster Schritt:** 
1. Committe die neuen Files (`.github/workflows/publish-package.yml`, `package.json`)
2. Pushe zu GitHub
3. Das Package wird automatisch beim nächsten Tag-Push erstellt!

Oder verwende Option B/C um manuell zu publishen.

---

**Möchtest du das Package jetzt veröffentlichen? 🚀**
