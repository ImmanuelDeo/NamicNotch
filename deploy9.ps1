$ErrorActionPreference = 'Continue'
Set-Location 'D:\NusaNotch-RE'
& cargo build --release *> build.log
"exit=$LASTEXITCODE"
Get-Content build.log | Select-String -Pattern '^error' -Context 0,4 | Select-Object -First 5 | ForEach-Object { $_.Line; $_.Context.PostContext }
if ($LASTEXITCODE -ne 0) { 'BUILD FAILED'; exit 1 }
Get-Content build.log | Select-String 'Finished' | Select-Object -Last 1
Get-Process nusa, namic, nusa-clone -ErrorAction SilentlyContinue | ForEach-Object { $_.Kill(); Start-Sleep -Milliseconds 300 }
Copy-Item 'target\release\nusa-clone.exe' 'D:\NamicNotch\nusa.exe' -Force
Copy-Item 'target\release\nusa-clone.exe' 'D:\NusaNotch-Portable\nusa.exe' -Force
Copy-Item 'target\release\nusa-clone.exe' 'D:\NusaNotch-Portable\namic.exe' -Force
"installed=$((Get-Item 'D:\NamicNotch\nusa.exe').Length) $((Get-Item 'D:\NamicNotch\nusa.exe').LastWriteTime) sha=$((Get-FileHash 'D:\NamicNotch\nusa.exe' -Algorithm SHA256).Hash.Substring(0,12))"
git add src/notch/renderer.rs src/mascot/eren.rs
git -c user.name='Immanuel Deo' -c user.email='deoiimmanuel59@gmail.com' commit -m "fix(notch): clip mascot aura to the pill and keep titan mane inside

The aura gradients are deliberately drawn past the pill bounds, so without a
clip they tinted the desktop around the notch. PushAxisAlignedClip now wraps
the idle mascot pass. Eren titan mane shortened so the silhouette stays
inside the pill even before clipping." 2>&1 | Select-Object -First 2
git log --oneline -1
