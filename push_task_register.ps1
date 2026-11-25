$ErrorActionPreference = 'Continue'
$name = 'NamicNotchPush'
Unregister-ScheduledTask -TaskName $name -Confirm:$false -ErrorAction SilentlyContinue
$action = New-ScheduledTaskAction -Execute 'powershell.exe' -Argument '-NoProfile -ExecutionPolicy Bypass -File D:\NusaNotch-RE\push_task.ps1'
$principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive -RunLevel Limited
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable -ExecutionTimeLimit (New-TimeSpan -Minutes 5)
Register-ScheduledTask -TaskName $name -Action $action -Principal $principal -Settings $settings -Force -ErrorAction SilentlyContinue | Out-Null
"registered=$([bool](Get-ScheduledTask -TaskName $name -ErrorAction SilentlyContinue))"
Remove-Item D:\NusaNotch-RE\task_run.log -ErrorAction SilentlyContinue
Start-ScheduledTask -TaskName $name
Start-Sleep -Seconds 35
"--- info ---"
Get-ScheduledTaskInfo -TaskName $name | Select-Object LastRunTime, LastTaskResult | Format-List | Out-String
"--- log ---"
Get-Content D:\NusaNotch-RE\task_run.log -ErrorAction SilentlyContinue
