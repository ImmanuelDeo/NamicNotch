$ErrorActionPreference = 'Continue'
# 1. Siapkan project snapshot
New-Item -ItemType Directory -Force -Path 'D:\NusaNotch-Snap' | Out-Null
Copy-Item 'D:\NusaNotch-RE\snaptop\*' 'D:\NusaNotch-Snap\' -Recurse -Force
Set-Location 'D:\NusaNotch-Snap'
& cargo build --release *> snap-build.log
"build_exit=$LASTEXITCODE"
Get-Content snap-build.log | Select-String -Pattern '^error' -Context 0,6 | Select-Object -First 8 | ForEach-Object { $_.Line; $_.Context.PostContext }
if ($LASTEXITCODE -ne 0) { exit 1 }
$dir = 'D:\NusaNotch-RE\snapshots'
New-Item -ItemType Directory -Force -Path $dir | Out-Null
& '.\target\release\namic-snapshots.exe' $dir
"run_exit=$LASTEXITCODE"
Get-ChildItem $dir -Filter *.png | ForEach-Object { "$($_.Name) $($_.Length)" }
