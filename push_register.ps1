$ErrorActionPreference = 'Continue'
$a = New-ScheduledTaskAction -Execute 'powershell.exe' -Argument '-NoProfile -ExecutionPolicy Bypass -File D:\NusaNotch-RE\push_now.ps1'
$p = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive -RunLevel Limited
Register-ScheduledTask -TaskName 'NamicNotchPush' -Action $a -Principal $p -Force | Out-Null
Start-ScheduledTask -TaskName 'NamicNotchPush'
Start-Sleep -Seconds 12
Get-Content 'D:\NusaNotch-RE\push_out.log' -ErrorAction SilentlyContinue
Unregister-ScheduledTask -TaskName 'NamicNotchPush' -Confirm:$false
Get-Item 'D:\NamicNotch\nusa.exe' | ForEach-Object { "INSTALLED $($_.Length) $($_.LastWriteTime.ToString('HH:mm:ss'))" }
