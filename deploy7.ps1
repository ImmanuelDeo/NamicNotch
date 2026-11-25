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
git add src/mascot src/notch/renderer.rs snaptop snap_run.ps1
git -c user.name='Immanuel Deo' -c user.email='deoiimmanuel59@gmail.com' commit -m "feat(mascot): true gradient brush, headless snapshot tool, taller Founding Titan

- draw_bottom_gradient now uses D2D1 linear gradient brush instead of stacked
  bands, so no ring/seam artifacts remain
- new snaptop/ crate renders every idle phase to a bitmap via WIC software
  target, used to document mascots without opening the app
- Eren titan: narrower and much taller skull plus a heavy mane curtain on
  each side, stronger recurring gold lightning" 2>&1 | Select-Object -First 2
git log --oneline -1
