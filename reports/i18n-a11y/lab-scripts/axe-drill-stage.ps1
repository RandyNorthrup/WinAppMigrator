# Axe.Windows on one stage of the AC-A11Y-AXE red drill: the app and the installer built for that stage.
param([Parameter(Mandatory)][ValidateSet('baseline', 'mutated', 'restored')][string]$Stage)
$folder = "C:\WamRepro\dl\axe-drill\$Stage"
"stage $Stage, from $folder"
& 'C:\WamRepro\scripts\axe-lab.ps1' -App "$folder\WinAppMigrator-drill.exe" -Stub "$folder\winappmigrator-installer-drill.exe" -Out "C:\WamRepro\logs\axe-drill-$Stage"
