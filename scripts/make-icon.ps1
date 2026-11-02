<#
.SYNOPSIS
    Regenerates the brand icon from code, so no design tool is needed to change it.

.DESCRIPTION
    Draws the same mark as `paint_logo` in crates/winappmigrator-ui/src/widgets.rs and
    writes the two files both executables use:

      crates/winappmigrator-ui/assets/winappmigrator.ico  (Explorer and the taskbar)
      crates/winappmigrator-ui/assets/winappmigrator.png  (the window title bar)
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Add-Type -AssemblyName System.Drawing

$assets = Join-Path $PSScriptRoot '..\crates\winappmigrator-ui\assets'
New-Item -ItemType Directory -Force -Path $assets | Out-Null

function New-LogoPng([int] $size) {
    $bitmap = New-Object System.Drawing.Bitmap($size, $size, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    try {
        $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
        $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
        $graphics.Clear([System.Drawing.Color]::Transparent)

        $corner = [float]($size * 0.22)
        $path = New-Object System.Drawing.Drawing2D.GraphicsPath
        $path.AddArc(0, 0, $corner * 2, $corner * 2, 180, 90)
        $path.AddArc($size - $corner * 2 - 1, 0, $corner * 2, $corner * 2, 270, 90)
        $path.AddArc($size - $corner * 2 - 1, $size - $corner * 2 - 1, $corner * 2, $corner * 2, 0, 90)
        $path.AddArc(0, $size - $corner * 2 - 1, $corner * 2, $corner * 2, 90, 90)
        $path.CloseFigure()
        $fill = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(45, 99, 224))
        $graphics.FillPath($fill, $path)

        $pen = New-Object System.Drawing.Pen([System.Drawing.Color]::White, [float][Math]::Max(1.5, $size * 0.095))
        $pen.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
        $pen.EndCap = [System.Drawing.Drawing2D.LineCap]::Round
        $pen.LineJoin = [System.Drawing.Drawing2D.LineJoin]::Round
        $left = [float]($size * 0.27); $middle = [float]($size * 0.5); $right = [float]($size * 0.73)
        $top = [float]($size * 0.27); $bottom = [float]($size * 0.73)
        $graphics.DrawLines($pen, [System.Drawing.PointF[]]@(
                [System.Drawing.PointF]::new($left, $top),
                [System.Drawing.PointF]::new($middle, $middle),
                [System.Drawing.PointF]::new($left, $bottom)))
        $graphics.DrawLines($pen, [System.Drawing.PointF[]]@(
                [System.Drawing.PointF]::new($middle, $top),
                [System.Drawing.PointF]::new($right, $middle),
                [System.Drawing.PointF]::new($middle, $bottom)))

        $stream = New-Object System.IO.MemoryStream
        $bitmap.Save($stream, [System.Drawing.Imaging.ImageFormat]::Png)
        return , $stream.ToArray()
    }
    finally {
        $graphics.Dispose()
        $bitmap.Dispose()
    }
}

$sizes = 16, 20, 24, 32, 40, 48, 64, 128, 256
$images = foreach ($size in $sizes) { , (New-LogoPng $size) }

[IO.File]::WriteAllBytes((Join-Path $assets 'winappmigrator.png'), (New-LogoPng 64))

$ico = New-Object System.IO.MemoryStream
$writer = New-Object System.IO.BinaryWriter($ico)
$writer.Write([uint16]0)
$writer.Write([uint16]1)
$writer.Write([uint16]$sizes.Count)
$offset = 6 + 16 * $sizes.Count
for ($index = 0; $index -lt $sizes.Count; $index++) {
    $size = $sizes[$index]
    $dimension = if ($size -ge 256) { 0 } else { $size }
    $writer.Write([byte]$dimension)
    $writer.Write([byte]$dimension)
    $writer.Write([byte]0)
    $writer.Write([byte]0)
    $writer.Write([uint16]1)
    $writer.Write([uint16]32)
    $writer.Write([uint32]$images[$index].Length)
    $writer.Write([uint32]$offset)
    $offset += $images[$index].Length
}
foreach ($image in $images) { $writer.Write($image) }
$writer.Flush()
[IO.File]::WriteAllBytes((Join-Path $assets 'winappmigrator.ico'), $ico.ToArray())

Write-Host "Wrote the icon to $((Resolve-Path $assets).Path)"
