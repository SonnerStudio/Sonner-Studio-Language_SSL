
Add-Type -AssemblyName System.Drawing

$sourcePath = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\FinalStoreHero.png"
$destPath = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\FinalStoreHero_16x9.png"

$sourceImg = [System.Drawing.Image]::FromFile($sourcePath)
$targetWidth = 1920
$targetHeight = 1080

$bmp = New-Object System.Drawing.Bitmap $targetWidth, $targetHeight
$graph = [System.Drawing.Graphics]::FromImage($bmp)
$graph.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$graph.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
$graph.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality

# 1. Fill background with blurred/scaled version
$bgRect = New-Object System.Drawing.Rectangle 0, 0, $targetWidth, $targetHeight
# Scale keeping aspect ratio to fill
$ratio = [Math]::Max($targetWidth / $sourceImg.Width, $targetHeight / $sourceImg.Height)
$bgW = $sourceImg.Width * $ratio
$bgH = $sourceImg.Height * $ratio
$bgX = ($targetWidth - $bgW) / 2
$bgY = ($targetHeight - $bgH) / 2

$graph.DrawImage($sourceImg, $bgX, $bgY, $bgW, $bgH)

# Overlay a dark semi-transparent rectangle to blur/dim background
$brush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(180, 0, 0, 0))
$graph.FillRectangle($brush, 0, 0, $targetWidth, $targetHeight)

# 2. Draw center image (Fit to height - padding)
$contentH = $targetHeight * 0.9
$contentRatio = $contentH / $sourceImg.Height
$contentW = $sourceImg.Width * $contentRatio
$contentX = ($targetWidth - $contentW) / 2
$contentY = ($targetHeight - $contentH) / 2

# Drop Shadow (Optional simple offset)
$shadowBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(100, 0, 0, 0))
$graph.FillRectangle($shadowBrush, ($contentX + 10), ($contentY + 10), $contentW, $contentH)

# Main Image
$graph.DrawImage($sourceImg, $contentX, $contentY, $contentW, $contentH)

$bmp.Save($destPath, [System.Drawing.Imaging.ImageFormat]::Png)

$sourceImg.Dispose()
$bmp.Dispose()
$graph.Dispose()
$brush.Dispose()
$shadowBrush.Dispose()

Write-Host "Created Landscape Image: $destPath"
