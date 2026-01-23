
$sourcePath = "C:\Users\hbcom\.gemini\antigravity\brain\37f2df63-b8b8-40a7-9d2b-41ca2b472275\test_logo_1768815718253.png"
$destDir = "msix\Assets"

function Resize-Image {
    param([string]$InputFile, [string]$OutputFile, [int]$Width, [int]$Height)
    
    Add-Type -AssemblyName System.Drawing
    $srcImage = [System.Drawing.Image]::FromFile($InputFile)
    $newBitmap = new-object System.Drawing.Bitmap($Width, $Height)
    $graphics = [System.Drawing.Graphics]::FromImage($newBitmap)
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.DrawImage($srcImage, 0, 0, $Width, $Height)
    $newBitmap.Save($OutputFile, [System.Drawing.Imaging.ImageFormat]::Png)
    $graphics.Dispose()
    $newBitmap.Dispose()
    $srcImage.Dispose()
    Write-Host "Generated $OutputFile"
}

Resize-Image -InputFile $sourcePath -OutputFile "$destDir\Square150x150Logo.png" -Width 150 -Height 150
Resize-Image -InputFile $sourcePath -OutputFile "$destDir\Square44x44Logo.png" -Width 44 -Height 44
Resize-Image -InputFile $sourcePath -OutputFile "$destDir\StoreLogo.png" -Width 50 -Height 50
