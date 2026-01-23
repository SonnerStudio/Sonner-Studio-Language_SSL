
Add-Type -AssemblyName System.Drawing

$sourcePath = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\FinalStoreHero.png"
$outputDir = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\ssl-v9"

function Create-Tile($size, $name) {
    $destPath = Join-Path $outputDir $name
    Write-Host "Generating Tile $size x $size ($name)..."

    $sourceImg = [System.Drawing.Image]::FromFile($sourcePath)
    $bmp = New-Object System.Drawing.Bitmap $size, $size
    $graph = [System.Drawing.Graphics]::FromImage($bmp)
    
    $graph.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graph.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $graph.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $graph.CompositingQuality = [System.Drawing.Drawing2D.CompositingQuality]::HighQuality

    # Since source and target are both square (or we want to fill), we draw full size
    # FinalStoreHero.png is expected to be square. If not, this might stretch, 
    # but based on previous context, the source is 1024x1024.
    
    $graph.DrawImage($sourceImg, 0, 0, $size, $size)

    $bmp.Save($destPath, [System.Drawing.Imaging.ImageFormat]::Png)
    
    $sourceImg.Dispose()
    $bmp.Dispose()
    $graph.Dispose()
}

# App Tile Icons (1:1)
Create-Tile 300 "Store_Tile_300x300.png"
Create-Tile 150 "Store_Tile_150x150.png"
Create-Tile 71  "Store_Tile_71x71.png"

Write-Host "All tiles generated successfully."
