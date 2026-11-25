$ErrorActionPreference = 'Continue'
Set-Location 'D:\NusaNotch-RE'
'=== build ==='
& cargo build --release *> build.log
"exit=$LASTEXITCODE"
Get-Content build.log | Select-String -Pattern '^error' -Context 0,4 | Select-Object -First 6 | ForEach-Object { $_.Line; $_.Context.PostContext }
if ($LASTEXITCODE -ne 0) { 'BUILD FAILED'; exit 1 }
Get-Content build.log | Select-String 'Finished' | Select-Object -Last 1

'=== deploy ==='
Get-Process nusa, namic, nusa-clone -ErrorAction SilentlyContinue | ForEach-Object { $_.Kill(); Start-Sleep -Milliseconds 300 }
Copy-Item 'target\release\nusa-clone.exe' 'D:\NamicNotch\nusa.exe' -Force
Copy-Item 'target\release\nusa-clone.exe' 'D:\NusaNotch-Portable\nusa.exe' -Force
Copy-Item 'target\release\nusa-clone.exe' 'D:\NusaNotch-Portable\namic.exe' -Force
"installed=$((Get-Item 'D:\NamicNotch\nusa.exe').Length) $((Get-Item 'D:\NamicNotch\nusa.exe').LastWriteTime) sha=$((Get-FileHash 'D:\NamicNotch\nusa.exe' -Algorithm SHA256).Hash.Substring(0,12))"

'=== commit ==='
git add src/mascot/sukuna.rs src/mascot/bakugo.rs src/mascot/eren.rs src/mascot/okarun.rs
git -c user.name='Immanuel Deo' -c user.email='deoiimmanuel59@gmail.com' commit -m "feat(mascot): match Asta Devil Union density for Sukuna, Bakugo, Eren, Okarun

- dark head base + rim light per character
- layered hair with highlight chunks and outer outline
- clothing collar (kimono, hero costume, Survey Corps cloak, hoodie)
- character props: Sukuna tattoos + second eyes, Bakugo mask/fins/gauntlet,
  Eren man-bun/titan marks, Okarun glasses/yokai lines
- keep eyes on shared illuminated capsule system" 2>&1 | Select-Object -First 3
git log --oneline -1
