; Nusa Notch Clone - Inno Setup Installer Script
; Compile with: iscc NusaNotchSetup.iss
; Requires Inno Setup 6.2+

[Setup]
AppId={{A8F3C2E1-7B4D-4F9A-8C3E-2F1D5E7B9A6C}
AppName=Nusa Notch Clone
AppVersion=1.0.0
AppPublisher=Nusa Notch Clone Project
AppPublisherURL=https://github.com/yourusername/nusa-clone
AppSupportURL=https://github.com/yourusername/nusa-clone/issues
AppUpdatesURL=https://github.com/yourusername/nusa-clone/releases
DefaultDirName={localappdata}\Programs\NusaNotch
DefaultGroupName=Nusa Notch Clone
AllowNoIcons=yes
OutputDir=..
OutputBaseFilename=NusaNotchSetup_v1.0.0
SetupIconFile=src\notch\resources\nusa.ico
Compression=lzma
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64
ArchitecturesAllowed=x64
PrivilegesRequired=lowest
UsePreviousAppDir=no
UsePreviousGroup=no
UsePreviousLanguage=no
DisableDirPage=no
DisableProgramGroupPage=no
DisableReadyPage=no
DisableReadyMemo=no
DisableFinishedPage=no
ShowLanguageDialog=auto
LicenseFile=LICENSE.txt
InfoBeforeFile=README.txt
; WizardImageFile=src\notch\resources\wizard.bmp
; WizardSmallImageFile=src\notch\resources\wizard_small.bmp

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
; Name: "indonesian"; MessagesFile: "compiler:Languages\Indonesian.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "startup"; Description: "Start with Windows (auto-launch on login)"; GroupDescription: "Options"
Name: "pinstart"; Description: "Pin to Start Menu"; GroupDescription: "Options"

[Files]
Source: "target\release\nusa-clone.exe"; DestDir: "{app}"; Flags: ignoreversion; DestName: "nusa.exe"
Source: "LICENSE.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Nusa Notch Clone"; Filename: "{app}\nusa.exe"; WorkingDir: "{app}"; Comment: "A living notch for Windows"
Name: "{group}\{cm:UninstallProgram,Nusa Notch Clone}"; Filename: "{uninstallexe}"; WorkingDir: "{app}"
Name: "{commondesktop}\Nusa Notch Clone"; Filename: "{app}\nusa.exe"; WorkingDir: "{app}"; Comment: "A living notch for Windows"; Tasks: desktopicon

[Registry]
; Auto-start with Windows
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "NusaNotchClone"; ValueData: "{app}\nusa.exe"; Flags: uninsdeletevalue; Tasks: startup
; App Paths for "nusa" command in Run dialog
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\App Paths\nusa.exe"; ValueType: string; ValueName: ""; ValueData: "{app}\nusa.exe"; Flags: uninsdeletekey

[Run]
Filename: "{app}\nusa.exe"; Description: "{cm:LaunchProgram,Nusa Notch Clone}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
Type: filesandordirs; Name: "{app}"

; [UninstallRun]
; Clean up registry

; [Code]
; Pascal Script for custom logic if needed

; [Messages]
; Custom messages for Indonesian
; indonesian.installing=Menginstall Nusa Notch Clone...
; indonesian.uninstalling=Menghapus Nusa Notch Clone...
; indonesian.ready=Siap untuk menginstall Nusa Notch Clone.
; indonesian.finished=Instalasi selesai.

[CustomMessages]
; Additional custom messages

[Components]
; Optional components (future use)

[Dirs]
; Ensure directories exist
Name: "{app}"; Permissions: users-modify

[InstallDelete]
; Clean up old versions
Type: files; Name: "{app}\*.*"

[INI]
; INI file entries (if needed)