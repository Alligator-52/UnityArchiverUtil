[Setup]
; Basic Information
AppName=Unity Archiver
AppVersion=1.0
DefaultDirName={pf}\UnityArchiver
DefaultGroupName=Unity Archiver
AllowNoIcons=yes
OutputDir=output
OutputBaseFilename=UnityArchiverInstaller
Compression=lzma
SolidCompression=yes
UninstallDisplayIcon={app}\unity_archiver.exe

[Files]
; Files to Install
Source: "D:\RustProjects\unity_archiver_util\target\release\unity_archiver.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "D:\RustProjects\unity_archiver_util\Icon.ico"; DestDir: "{app}"; Flags: ignoreversion
Source: "D:\RustProjects\unity_archiver_util\ico16.ico"; DestDir:"{app}"; Flags: ignoreversion 

[Icons]
; Start Menu Shortcut
Name: "{group}\Unity Archiver"; Filename: "{app}\unity_archiver.exe"; IconFilename: "{app}\Icon.ico"

[UninstallDelete]
; Delete Custom Files on Uninstall (if any)
Type: files; Name: "{app}\unity_archiver.exe"
Type: files; Name: "{app}\Icon.ico"

[Run]
; Run Application After Installation
Filename: "{app}\unity_archiver.exe"; Description: "Launch Unity Archiver"; Flags: nowait postinstall skipifsilent

[Registry]
; Add Uninstall Information to Control Panel
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\UnityArchiver"; Flags: uninsdeletekeyifempty
Root: HKCU; Subkey: "Software\UnityArchiver"; ValueType: string; ValueName: "InstallPath"; ValueData: "{app}"; Flags: uninsdeletevalue

;Add Right-Click Context Menu Entry for Right-Click on a folder
Root: HKCR; Subkey: "Directory\shell\ArchiveWithUnity"; ValueType: string; ValueData: "Archive Unity Project"
Root: HKCR; Subkey: "Directory\shell\ArchiveWithUnity\command"; ValueType: string; ValueData: """cmd.exe"" /C """"{app}\unity_archiver.exe"" ""%1"""""
;Root: HKCR; Subkey: "Directory\shell\ArchiveWithUnity\command"; ValueType: string; ValueData: """C:\Program Files\Git\bin\bash.exe"" -c ""'{app}/unity_archiver.exe' '%1'"""
Root: HKCR; Subkey: "Directory\shell\ArchiveWithUnity\Icon"; ValueType: string; ValueData: "{app}\1co16.ico"

; Add Right-Click Context Menu Entry for Right-Click Inside a Directory
Root: HKCR; Subkey: "Directory\Background\shell\ArchiveWithUnity"; ValueType: string; ValueData: "Archive Unity Project"
Root: HKCR; Subkey: "Directory\Background\shell\ArchiveWithUnity\command"; ValueType: string; ValueData: """cmd.exe"" /C """"{app}\unity_archiver.exe"" ""%V"""""
;Root: HKCR; Subkey: "Directory\Background\shell\ArchiveWithUnity\command"; ValueType: string; ValueData: """C:\Program Files\Git\bin\bash.exe"" -c ""'{app}/unity_archiver.exe' '%V'"""
Root: HKCR; Subkey: "Directory\Background\shell\ArchiveWithUnity\Icon"; ValueType: string; ValueData: "{app}\ico16.ico"

[Tasks]
; Optional Tasks: Create Desktop Shortcut
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
