param(
    [string]$sourceExe,
    [string]$outputIco
)

Add-Type -AssemblyName "System.Drawing"
#$sourceExe = "D:\software\develop\jetbrains\RustRover\bin\rustrover64.exe"
#$outputIco = "C:\Users\Silwings\AppData\Roaming\JetBrains\RustRover2024.2\scratches\SaveIcon.png"

Write-Host "Source EXE: $sourceExe"
Write-Host "Output ICO: $outputIco"

if (-Not (Test-Path -Path $sourceExe)) {
    Write-Host "The source file does not exist: $sourceExe"
    exit 1
}

$icon = [System.Drawing.Icon]::ExtractAssociatedIcon($sourceExe)

if ($icon -ne $null) {
    $bitmap = $icon.ToBitmap()
    $bitmap.Save($outputIco, [System.Drawing.Imaging.ImageFormat]::Icon)
} else {
    Write-Host "Unable to extract icons from the specified application: $sourceExe"
    exit 1
}