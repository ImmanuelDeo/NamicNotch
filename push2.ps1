$ErrorActionPreference = 'Continue'
Set-Location 'D:\NusaNotch-RE'
"--- stored creds ---"
cmdkey /list | Select-String -Pattern 'github' | Select-Object -First 5
"--- helpers ---"
git config --get-all credential.helper
"--- files ---"
"git-credentials=$(Test-Path (Join-Path $env:USERPROFILE '.git-credentials'))"
$env:GIT_TERMINAL_PROMPT = '0'
"--- try manager dpapi ---"
git -c credential.helper=manager -c credential.credentialStore=dpapi push origin main 2>&1 | Select-Object -Last 4
"exit1=$LASTEXITCODE"
"--- try store ---"
git -c credential.helper=store push origin main 2>&1 | Select-Object -Last 4
"exit2=$LASTEXITCODE"
