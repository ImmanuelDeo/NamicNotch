$ErrorActionPreference = 'Continue'
Set-Location 'D:\NusaNotch-RE'
git push origin HEAD:main 2>&1 | Out-File -Encoding utf8 'D:\NusaNotch-RE\push_out.log'
"EXIT=$LASTEXITCODE" | Out-File -Encoding utf8 -Append 'D:\NusaNotch-RE\push_out.log'
