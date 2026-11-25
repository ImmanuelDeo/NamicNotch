$ErrorActionPreference = 'Continue'
Set-Location 'D:\NusaNotch-RE'
"--- status ---"
git status --porcelain | Select-Object -First 12
"--- add ---"
git add -A src/mascot
git -c user.name='Immanuel Deo' -c user.email='deoiimmanuel59@gmail.com' commit -m "fix(mascot): lift head base value so silhouettes read on obsidian" 2>&1 | Select-Object -First 3
"--- head ---"
git log --oneline -2
