$ErrorActionPreference = 'Continue'
Set-Location 'D:\NusaNotch-RE'
& cargo build --release *> build.log
"exit=$LASTEXITCODE"
Get-Content build.log | Select-String -Pattern '^error' -Context 0,4 | Select-Object -First 6 | ForEach-Object { $_.Line; $_.Context.PostContext }
if ($LASTEXITCODE -ne 0) { 'BUILD FAILED'; exit 1 }
Get-Content build.log | Select-String 'Finished' | Select-Object -Last 1
Get-Process nusa, namic, nusa-clone -ErrorAction SilentlyContinue | ForEach-Object { $_.Kill(); Start-Sleep -Milliseconds 300 }
Copy-Item 'target\release\nusa-clone.exe' 'D:\NamicNotch\nusa.exe' -Force
Copy-Item 'target\release\nusa-clone.exe' 'D:\NusaNotch-Portable\nusa.exe' -Force
Copy-Item 'target\release\nusa-clone.exe' 'D:\NusaNotch-Portable\nusamic.exe' -Force -ErrorAction SilentlyContinue
Copy-Item 'target\release\nusa-clone.exe' 'D:\NusaNotch-Portable\namic.exe' -Force
"installed=$((Get-Item 'D:\NamicNotch\nusa.exe').Length) $((Get-Item 'D:\NamicNotch\nusa.exe').LastWriteTime) sha=$((Get-FileHash 'D:\NamicNotch\nusa.exe' -Algorithm SHA256).Hash.Substring(0,12))"
git add src/mascot/core.rs src/mascot/asta.rs src/mascot/eren.rs src/mascot/sukuna.rs src/mascot/bakugo.rs src/mascot/okarun.rs
git -c user.name='Immanuel Deo' -c user.email='deoiimmanuel59@gmail.com' commit -m "feat(mascot): organic faces replace head box; per-character VFX pass

- draw_smooth_face: single smooth loop with jaw shading, coloured skin per
  character instead of the black rounded-rect that read as a cloak
- Asta: crimson thunder branches plus anti-magic smoke
- Sukuna: white crescent Dismantle slashes with red rim, crimson and violet
  flame ribbons
- Bakugo: centre impact starburst with hot core, dark smoke puffs, ember rays
- Eren: mint steam and gold branching titan lightning, Founding Titan face
- Okarun: long crimson swooshes, white highlight bands, pink edge crackles" 2>&1 | Select-Object -First 2
git log --oneline -1
