; Inno Setup Script
[Setup]
AppName=Becksman
AppVersion=1.0
DefaultDirName={commonpf}\Becksman
DefaultGroupName=Becksman
UninstallDisplayIcon={app}\assets\icon.ico
OutputDir=.
OutputBaseFilename=Becksman_Installer
Compression=lzma
SolidCompression=yes

[Files]
; Install becks_client.exe and becks_server.exe to app/bin/
Source: ".\target\release\becks_client.exe"; DestDir: "{app}\bin"; Flags: ignoreversion
Source: ".\target\release\becks_server.exe"; DestDir: "{app}\bin"; Flags: ignoreversion

; Install Entry.exe to app/
Source: ".\Entry\x64\Debug\Entry.exe"; DestDir: "{app}"; Flags: ignoreversion

; Install sqlite3.dll to app/
Source: ".\binary\sqlite3.dll"; DestDir: "{app}/bin"; Flags: ignoreversion

; Install all contents under ./assets to app/assets/
Source: ".\assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\Becksman"; Filename: "{app}\Entry.exe"
Name: "{group}\Uninstall Becksman"; Filename: "{uninstallexe}"

[Run]
; Optional: Run the client after installation
Filename: "{app}\Entry.exe"; Description: "Launch Becksman"; Flags: nowait postinstall skipifsilent