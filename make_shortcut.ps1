$exe = 'D:\NamicNotch\nusa.exe'
$ws = New-Object -ComObject WScript.Shell
$targets = @(
    (Join-Path ([Environment]::GetFolderPath('Desktop')) 'Namic Notch.lnk'),
    (Join-Path ([Environment]::GetFolderPath('Programs')) 'Namic Notch.lnk')
)
foreach ($t in $targets) {
    $s = $ws.CreateShortcut($t)
    $s.TargetPath = $exe
    $s.WorkingDirectory = 'D:\NamicNotch'
    $s.IconLocation = 'D:\NamicNotch\icon.ico'
    $s.Description = 'Namic Notch'
    $s.Save()
    "shortcut: $t"
}
"installed=$((Get-Item $exe).LastWriteTime)"
