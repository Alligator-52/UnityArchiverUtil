@echo off
REM Path to Git Bash
set GITBASH="C:\Program Files\Git\bin\bash.exe"

REM Path to your executable
set TOOL_PATH="%~dp0unity_archiver.exe"

REM Check if Git Bash exists
if exist %GITBASH% (
    REM Run in Git Bash
    %GITBASH% -c "%TOOL_PATH% %1"
) else (
    REM Run in Command Prompt
    cmd.exe /K "%TOOL_PATH% %1"
)