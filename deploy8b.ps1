$ErrorActionPreference = 'Continue'
Set-Location 'D:\NusaNotch-RE'
git add src/mascot/sukuna.rs src/mascot/bakugo.rs src/mascot/okarun.rs src/mascot/eren.rs
git -c user.name='Immanuel Deo' -c user.email='deoiimmanuel59@gmail.com' commit -m "fix(mascot): lift head base contrast so silhouettes read on obsidian" 2>&1 | Select-Object -First 2
git log --oneline -1
& 'D:\NusaNotch-Snap\src\..\target\release\namic-snapshots.exe' 'D:\NusaNotch-RE\snapshots' 2>&1 | Select-Object -Last 1
