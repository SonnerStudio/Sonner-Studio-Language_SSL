
Add-Type -AssemblyName System.Drawing

$sourcePath = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\FinalStoreHero.png"
$outputDir = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL"

function Create-Art($targetW, $targetH, $name) {
    $destPath = Join-Path $outputDir $name
    Write-Host "Generating $name ($targetW x $targetH)..."

    $sourceImg = [System.Drawing.Image]::FromFile($sourcePath)
    $bmp = New-Object System.Drawing.Bitmap $targetW, $targetH
    $graph = [System.Drawing.Graphics]::FromImage($bmp)
    
    $graph.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graph.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $graph.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $graph.CompositingQuality = [System.Drawing.Drawing2D.CompositingQuality]::HighQuality

    # Calculate Aspect Ratios
    $sourceRatio = $sourceImg.Width / $sourceImg.Height
    $targetRatio = $targetW / $targetH

    if ([Math]::Abs($sourceRatio - $targetRatio) -lt 0.01) {
        # Direct Scaling (Square to Square)
        $graph.DrawImage($sourceImg, 0, 0, $targetW, $targetH)
    } else {
        # Composition with Blurred Background
        
        # 1. Background Fill (Scaled up to cover)
        $scale = [Math]::Max($targetW / $sourceImg.Width, $targetH / $sourceImg.Height)
        $bgW = $sourceImg.Width * $scale
        $bgH = $sourceImg.Height * $scale
        $bgX = ($targetW - $bgW) / 2
        $bgY = ($targetH - $bgH) / 2
        
        $graph.DrawImage($sourceImg, $bgX, $bgY, $bgW, $bgH)

        # 2. Blur / Dim Overlay
        $brush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(160, 0, 0, 0))
        $graph.FillRectangle($brush, 0, 0, $targetW, $targetH)

        # 3. Center Content (Fit within bounds with padding)
        $padding = [Math]::Min($targetW, $targetH) * 0.1
        $availW = $targetW - ($padding * 2)
        $availH = $targetH - ($padding * 2)
        
        $fitScale = [Math]::Min($availW / $sourceImg.Width, $availH / $sourceImg.Height)
        $fgW = $sourceImg.Width * $fitScale
        $fgH = $sourceImg.Height * $fitScale
        $fgX = ($targetW - $fgW) / 2
        $fgY = ($targetH - $fgH) / 2

        # Drop Shadow
        $shadowBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(80, 0, 0, 0))
        $graph.FillRectangle($shadowBrush, ($fgX + 10), ($fgY + 10), $fgW, $fgH)

        $graph.DrawImage($sourceImg, $fgX, $fgY, $fgW, $fgH)
    }

    $bmp.Save($destPath, [System.Drawing.Imaging.ImageFormat]::Png)
    
    $sourceImg.Dispose()
    $bmp.Dispose()
    $graph.Dispose()
    $brush.Dispose()
    $shadowBrush.Dispose()
}

# Poster Art (Portrait)
Create-Art 720 1080 "Store_Poster_720x1080.png"
Create-Art 1440 2160 "Store_Poster_1440x2160.png"

# Box Art (Square)
Create-Art 1080 1080 "Store_BoxArt_1080x1080.png"
Create-Art 2160 2160 "Store_BoxArt_2160x2160.png"
