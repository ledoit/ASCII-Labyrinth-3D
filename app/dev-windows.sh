#!/bin/bash
# Development script for Windows that ensures proper MinGW environment for GNU toolchain

# Add MinGW to PATH for dlltool and other tools
export PATH="/c/msys64/mingw64/bin:$PATH"

# Remove Git usr/bin from PATH to avoid link.exe conflict (if using MSVC)
export PATH=$(echo "$PATH" | sed 's|/c/Program Files/Git/usr/bin:||g')
export PATH=$(echo "$PATH" | sed 's|:/c/Program Files/Git/usr/bin||g')

# Run the dev command
npm run tauri dev
