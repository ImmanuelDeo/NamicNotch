; Namic Notch - Inno Setup Installer Script
; Compile with: iscc NamicNotchSetup.iss / iscc NusaNotchSetup.iss
; Requires Inno Setup 6.2+

[Setup]
AppId={{A8F3C2E1-7B4D-4F9A-8C3E-2F1D5E7B9A6C}
AppName=Namic Notch
AppVersion=1.0.0
AppPublisher=Namic Notch Project
AppPublisherURL=https://github.com/yourusername/namic-notch
AppSupportURL=https://github.com/yourusername/namic-notch/issues
AppUpdatesURL=https://github.com/yourusername/namic-notch/releases
DefaultDirName={localappdata}\Programs\NamicNotch
DefaultGroupName=Namic Notch
AllowNoIcons=yes
OutputDir=..
OutputBaseFilename=NamicNotchSetup_v1.0.0
SetupIconFile=resources\icon.ico
Compression=lzma
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible
ArchitecturesAllowed=x64compatible
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
; WizardImageFile=resources\wizard.bmp
; WizardSmallImageFile=resources\wizard_small.bmp

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
Name: "{group}\Namic Notch"; Filename: "{app}\nusa.exe"; WorkingDir: "{app}"; Comment: "A living dynamic notch for Windows"
Name: "{group}\{cm:UninstallProgram,Namic Notch}"; Filename: "{uninstallexe}"; WorkingDir: "{app}"
Name: "{userdesktop}\Namic Notch"; Filename: "{app}\nusa.exe"; WorkingDir: "{app}"; Comment: "A living dynamic notch for Windows"; Tasks: desktopicon
Name: "{userstartup}\Namic Notch"; Filename: "{app}\nusa.exe"; WorkingDir: "{app}"; Comment: "A living dynamic notch for Windows"; Tasks: startup

[Registry]
; Auto-start with Windows
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "NamicNotch"; ValueData: "{app}\nusa.exe"; Flags: uninsdeletevalue; Tasks: startup
; App Paths for "nusa" command in Run dialog
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\App Paths\nusa.exe"; ValueType: string; ValueName: ""; ValueData: "{app}\nusa.exe"; Flags: uninsdeletekey

[Run]
Filename: "{app}\nusa.exe"; Description: "{cm:LaunchProgram,Namic Notch}"; Flags: nowait postinstall skipifsilent

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