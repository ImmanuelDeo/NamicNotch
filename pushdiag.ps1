$ErrorActionPreference = 'Continue'
"--- ssh keys ---"
Get-ChildItem "$env:USERPROFILE\.ssh" -ErrorAction SilentlyContinue | Select-Object Name | ForEach-Object { $_.Name }
"--- gh ---"
$gh = Get-Command gh -ErrorAction SilentlyContinue
if ($gh) {
  gh auth status 2>&1 | Select-Object -First 6
  $tok = gh auth token 2>$null
  if ($tok) { "gh_token_len=$($tok.Length)" } else { "gh_token=missing" }
} else { "gh=not_installed" }
"--- remote ---"
Set-Location 'D:\NusaNotch-RE'
git remote -v
"--- ssh test ---"
ssh -o StrictHostKeyChecking=no -o BatchMode=yes -T git@github.com 2>&1 | Select-Object -First 2
