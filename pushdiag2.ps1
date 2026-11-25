$ErrorActionPreference = 'Continue'
"--- keys ---"
if (Test-Path "$env:USERPROFILE\.ssh") {
  Get-ChildItem "$env:USERPROFILE\.ssh" -File | ForEach-Object { $_.Name }
} else { "no .ssh dir" }
"--- ssh auth ---"
$out = & ssh -o StrictHostKeyChecking=no -o BatchMode=yes -T git@github.com 2>&1
"output: $out"
"exit: $LASTEXITCODE"
