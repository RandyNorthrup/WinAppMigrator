# AC-RELEASE: the downloaded app starts in the language Windows is set to. For each language, the signed-in account's
# display-language list (HKCU\Control Panel\Desktop\PreferredUILanguages, which Settings writes) is set to it alone, with
# no language chosen in the app, and the app is started; its window's controls are read through UI Automation and a
# picture is kept. The list and any choice saved in the app are put back afterward.
param([string]$Exe = 'C:\WamRepro\dl\release\WinAppMigrator-v0.7.0.exe', [string]$Out = 'C:\WamRepro\logs\release-languages')
$ErrorActionPreference = 'Stop'
. C:\WamRepro\scripts\uia.ps1
New-Item -ItemType Directory -Force -Path $Out | Out-Null
$desktop = 'HKCU:\Control Panel\Desktop'
$listBefore = (Get-ItemProperty -LiteralPath $desktop -Name PreferredUILanguages -ErrorAction SilentlyContinue).PreferredUILanguages
$choice = Join-Path $env:LOCALAPPDATA 'WinAppMigrator\language.txt'
$choiceBefore = if (Test-Path -LiteralPath $choice) { Get-Content -LiteralPath $choice -Raw } else { $null }
$name = [IO.Path]::GetFileNameWithoutExtension($Exe)
"app: $Exe"
"sha256: $((Get-FileHash -Algorithm SHA256 -LiteralPath $Exe).Hash)"
"display-language list before: $(if ($listBefore) { $listBefore -join ', ' } else { '(none)' }); choice saved in the app: $(if ($null -ne $choiceBefore) { $choiceBefore } else { '(none)' })"
try {
    Remove-Item -LiteralPath $choice -ErrorAction SilentlyContinue
    foreach ($tag in 'it-IT', 'ar-SA') {
        Set-ItemProperty -LiteralPath $desktop -Name PreferredUILanguages -Value ([string[]]@($tag)) -Type MultiString
        Get-Process -Name $name -ErrorAction SilentlyContinue | Stop-Process -Force
        Start-Sleep -Seconds 2
        Start-Process -FilePath $Exe
        $w = Get-WamWindow 'WinAppMigrator*' 90
        Show-WamWindow $w
        Start-Sleep -Seconds 20
        "=== Windows set to ${tag}: window '$($w.Current.Name)'"
        Get-WamElements $w | Where-Object {
            $_.Current.ControlType.ProgrammaticName -in 'ControlType.Button', 'ControlType.ComboBox', 'ControlType.Edit', 'ControlType.Text' -and $_.Current.Name
        } | Select-Object -First 14 | ForEach-Object { "  $($_.Current.ControlType.ProgrammaticName.Replace('ControlType.', '')): $($_.Current.Name)" }
        Save-WamWindowShot $w (Join-Path $Out "$tag.png")
        # An app's details, where sizes, versions and paths sit inside sentences in the window's language.
        $row = Get-WamElements $w | Where-Object { $_.Current.Name -like '*7-Zip*' -or $_.Current.Name -like '*PuTTY*' } | Select-Object -First 1
        if ($row) {
            Invoke-WamElement $row
            Start-Sleep -Seconds 15
            "  picked: $($row.Current.Name)"
            Save-WamWindowShot $w (Join-Path $Out "$tag-app.png")
        }
        Get-Process -Name $name -ErrorAction SilentlyContinue | Stop-Process -Force
    }
} finally {
    Get-Process -Name $name -ErrorAction SilentlyContinue | Stop-Process -Force
    if ($null -eq $listBefore) { Remove-ItemProperty -LiteralPath $desktop -Name PreferredUILanguages -ErrorAction SilentlyContinue }
    else { Set-ItemProperty -LiteralPath $desktop -Name PreferredUILanguages -Value ([string[]]$listBefore) -Type MultiString }
    if ($null -ne $choiceBefore) { Set-Content -LiteralPath $choice -Value $choiceBefore -NoNewline }
    "display-language list after: $(((Get-ItemProperty -LiteralPath $desktop -Name PreferredUILanguages -ErrorAction SilentlyContinue).PreferredUILanguages) -join ', ')"
}
