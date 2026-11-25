$ErrorActionPreference = 'Continue'
Set-Location 'D:\NusaNotch-RE'
"start $(Get-Date -Format o)" | Out-File -Encoding utf8 D:\NusaNotch-RE\task_run.log
$env:GIT_TERMINAL_PROMPT = '0'
& git push origin main *>> D:\NusaNotch-RE\task_run.log
"exit=$LASTEXITCODE" | Out-File -Append -Encoding utf8 D:\NusaNotch-RE\task_run.log
& git rev-parse --short HEAD | Out-File -Append -Encoding utf8 D:\NusaNotch-RE\task_run.log
