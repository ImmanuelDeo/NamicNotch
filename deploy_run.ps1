$ErrorActionPreference = 'Continue'
$repo = 'D:\NusaNotch-RE'
$install = 'D:\NamicNotch'
$exe = Join-Path $install 'nusa.exe'
$log = Join-Path $repo 'build.log'

'=== BUILD ==='
Push-Location $repo
& cargo build --release *> $log
$code = $LASTEXITCODE
Pop-Location
"exit=$code"
Get-Content $log | Select-String -Pattern '^error' | Select-Object -First 10
Get-Content $log | Select-String -Pattern 'Finished' | Select-Object -Last 1
if ($code -ne 0) { 'BUILD FAILED'; exit 1 }

$built = Join-Path $repo 'target\release\nusa-clone.exe'
"built=$((Get-Item $built).Length) bytes  $((Get-Item $built).LastWriteTime)"

'=== STOP OLD ==='
Get-Process nusa, namic, nusa-clone -ErrorAction SilentlyContinue | ForEach-Object { "stop $($_.Id) $($_.ProcessName)"; $_.Kill(); Start-Sleep -Milliseconds 300 }

'=== INSTALL ==='
if (Test-Path $install) {
    $unins = Get-ChildItem $install -Filter 'unins*.exe' -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($unins) { "uninstall silend"; & $unins.FullName /VERYSILENT /SUPPRESSMSGBOXES /NORESTART | Out-Null; Start-Sleep -Seconds 3 }
    Remove-Item $install -Recurse -Force -ErrorAction SilentlyContinue
}
New-Item -ItemType Directory -Path $install -Force | Out-Null
Copy-Item $built $exe -Force
foreach ($r in @('icon.ico', 'icon.png')) {
    $s = Join-Path $repo "resources\$r"
    if (Test-Path $s) { Copy-Item $s (Join-Path $install $r) -Force }
}
Copy-Item $built 'D:\NusaNotch-Portable\nusa.exe' -Force -ErrorAction SilentlyContinue
Copy-Item $built 'D:\NusaNotch-Portable\namic.exe' -Force -ErrorAction SilentlyContinue
"installed=$((Get-Item $exe).Length) bytes  sha=$((Get-FileHash $exe -Algorithm SHA256).Hash.Substring(0,16))"

'=== AUTOSTART (single path) ==='
$runKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'
New-Item -Path $runKey -Force | Out-Null
Set-ItemProperty -Path $runKey -Name 'NamicNotch' -Value ('"' + $exe + '"')
$lnk = Join-Path ([System.IO.Path]::Combine($env:APPDATA, 'Microsoft\Windows\Start Menu\Programs\Startup')) 'Namic Notch.lnk'
if (Test-Path $lnk) { Remove-Item $lnk -Force; 'removed duplicate startup shortcut' }
"Run\NamicNotch=$((Get-ItemProperty $runKey).NamicNotch)"
"startup shortcut exists=$(Test-Path $lnk)"

'=== LAUNCH PROBE ==='
$out = Join-Path $repo 'run_out.txt'
$err = Join-Path $repo 'run_err.txt'
Remove-Item $out, $err -ErrorAction SilentlyContinue
$p = Start-Process -FilePath $exe -RedirectStandardOutput $out -RedirectStandardError $err -PassThru
Start-Sleep -Seconds 6
if ($p.HasExited) { "EXITED code=$($p.ExitCode)"; Get-Content $err -ErrorAction SilentlyContinue | Select-Object -Last 5 }
else { "RUNNING PID=$($p.Id)" }
'DONE'
