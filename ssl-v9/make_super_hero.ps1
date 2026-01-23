
Add-Type -AssemblyName System.Drawing

$sourcePath = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\FinalStoreHero.png"
$outputDir = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\ssl-v9"

function Create-Composition($targetW, $targetH, $name) {
    $destPath = Join-Path $outputDir $name
    Write-Host "Generating Super Hero $targetW x $targetH ($name)..."

    $sourceImg = [System.Drawing.Image]::FromFile($sourcePath)
    $bmp = New-Object System.Drawing.Bitmap $targetW, $targetH
    $graph = [System.Drawing.Graphics]::FromImage($bmp)
    
    $graph.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graph.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $graph.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $graph.CompositingQuality = [System.Drawing.Drawing2D.CompositingQuality]::HighQuality

    # 1. Background Fill (Scaled up to cover)
    $scale = [Math]::Max($targetW / $sourceImg.Width, $targetH / $sourceImg.Height)
    $bgW = $sourceImg.Width * $scale
    $bgH = $sourceImg.Height * $scale
    $bgX = ($targetW - $bgW) / 2
    $bgY = ($targetH - $bgH) / 2
    
    $graph.DrawImage($sourceImg, $bgX, $bgY, $bgW, $bgH)

    # 2. Blur / Dim Overlay
    $brush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(180, 0, 0, 0))
    $graph.FillRectangle($brush, 0, 0, $targetW, $targetH)

    # 3. Center Content (Fit within bounds with padding)
    # For Super Hero Art, we want the logo/centerpiece to be prominent.
    # We use 85% of height as guideline.
    $contentH = $targetH * 0.85
    $contentRatio = $contentH / $sourceImg.Height
    $contentW = $sourceImg.Width * $contentRatio
    $contentX = ($targetW - $contentW) / 2
    $contentY = ($targetH - $contentH) / 2

    # Drop Shadow
    $shadowBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(100, 0, 0, 0))
    $graph.FillRectangle($shadowBrush, ($contentX + 15), ($contentY + 15), $contentW, $contentH)

    $graph.DrawImage($sourceImg, $contentX, $contentY, $contentW, $contentH)

    $bmp.Save($destPath, [System.Drawing.Imaging.ImageFormat]::Png)
    
    $sourceImg.Dispose()
    $bmp.Dispose()
    $graph.Dispose()
    $brush.Dispose()
    $shadowBrush.Dispose()
}

# Super Hero Art (16:9)
Create-Composition 1920 1080 "Store_SuperHero_1920x1080.png"
Create-Composition 3840 2160 "Store_SuperHero_3840x2160.png" # 4K

Write-Host "Super Hero assets generated."
