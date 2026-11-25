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
Copy-Item 'target\release\nusa-clone.exe' 'D:\NusaNotch-Portable\namic.exe' -Force
"installed=$((Get-Item 'D:\NamicNotch\nusa.exe').Length) $((Get-Item 'D:\NamicNotch\nusa.exe').LastWriteTime) sha=$((Get-FileHash 'D:\NamicNotch\nusa.exe' -Algorithm SHA256).Hash.Substring(0,12))"
git add src/mascot/core.rs src/mascot/eren.rs src/mascot/sukuna.rs src/mascot/bakugo.rs src/mascot/okarun.rs
git -c user.name='Immanuel Deo' -c user.email='deoiimmanuel59@gmail.com' commit -m "feat(mascot): reference-accurate environments and Founding Titan Eren

- new effect primitives: bottom gradient bands, curved smoke wisps,
  full-width speed lines, claw slashes, branching anime lightning
- Sukuna: crimson core glow with magenta-purple smoke flanking the head
- Bakugo: amber core glow, 12 drifting embers, soft double shockwave
- Okarun: layered red/white speed lines, spectral claws, low haze
- Eren titan: flat Founding Titan silhouette, elongated skull, long mane
  past the jaw, single jagged teeth row, thin titan marks, stretched green eyes" 2>&1 | Select-Object -First 2
git log --oneline -1
