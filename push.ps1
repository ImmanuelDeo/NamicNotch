$ErrorActionPreference = 'Continue'
Set-Location 'D:\NusaNotch-RE'
$env:GIT_TERMINAL_PROMPT = '0'
"--- branch ---"
git rev-parse --abbrev-ref HEAD
git log --oneline -1
"--- push attempt ---"
git push origin main 2>&1 | Select-Object -Last 6
"push_exit=$LASTEXITCODE"
git status -sb | Select-Object -First 1
