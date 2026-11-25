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
git add src/mascot snaptop
git -c user.name='Immanuel Deo' -c user.email='deoiimmanuel59@gmail.com' commit -m "feat(mascot): VFX v3.0 taper pipeline, no skin faces, unified capsule eyes

- new src/mascot/vfx.rs: tapered ribbon fills (no constant-width strokes),
  6-tier bloom, branching tapered lightning, flame ribbons, crescent slashes,
  connected 24-point explosion, feathered steam, ambient illumination, bokeh
- draw_soft_circle now built on a 4-stop radial gradient brush, so smoke and
  haze have no hard ring edges anywhere
- all five characters blend into the obsidian notch: skin-tone faces removed,
  replaced by a near-black head base; cartoon teeth and bar eyebrows gone
- Asta: pupil cross removed, capsule eyes shared, lightning tapered
- Sukuna: hard ring gesture replaced by soft glow plus tapered arcs" 2>&1 | Select-Object -First 2
git log --oneline -1
