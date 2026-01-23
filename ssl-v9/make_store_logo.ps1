
Add-Type -AssemblyName System.Drawing

$sourcePath = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\FinalStoreHero.png"
$destPath = "c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\ssl-v9\msix\Assets\StoreLogo.png"

Write-Host "Generating StoreLogo.png (50x50)..."

$sourceImg = [System.Drawing.Image]::FromFile($sourcePath)
$bmp = New-Object System.Drawing.Bitmap 50, 50
$graph = [System.Drawing.Graphics]::FromImage($bmp)
$graph.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$graph.DrawImage($sourceImg, 0, 0, 50, 50)
$bmp.Save($destPath, [System.Drawing.Imaging.ImageFormat]::Png)

$sourceImg.Dispose()
$bmp.Dispose()
$graph.Dispose()

Write-Host "Done."
