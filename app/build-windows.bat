@echo off
REM Build script for Windows that ensures proper MinGW environment for GNU toolchain
setlocal

REM Add MinGW to PATH for dlltool and other tools
set "PATH=C:\msys64\mingw64\bin;%PATH%"

REM Remove Git usr/bin from PATH to avoid link.exe conflict (if using MSVC)
set "PATH=%PATH:C:\Program Files\Git\usr\bin;=%"
set "PATH=%PATH:;C:\Program Files\Git\usr\bin=%"

REM Run the build
call npm run tauri build

endlocal

