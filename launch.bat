@echo off
title Text Adventure Launcher

REM Clear screen and show welcome message
cls
echo ================================
echo    Launching Text Adventure...
echo ================================
echo.
echo A new command window will open with the game.
echo Close that window to exit the game.
echo.

REM Get the current directory
set DIR=%~dp0

REM Set up registry keys for console font and window size (temporary)
REG ADD "HKCU\Console\Text Adventure" /v FaceName /t REG_SZ /d "Consolas" /f
REG ADD "HKCU\Console\Text Adventure" /v FontSize /t REG_DWORD /d 0x000e0000 /f
REG ADD "HKCU\Console\Text Adventure" /v WindowSize /t REG_DWORD /d 0x00230078 /f
REG ADD "HKCU\Console\Text Adventure" /v ScreenBufferSize /t REG_DWORD /d 0x03e80078 /f

REM Launch the game in a new window with custom colors
start "Text Adventure" cmd /c "mode con: cols=120 lines=35 && cd /d "%DIR%" && color 0a && cls && cargo run && pause"

echo Game launched successfully!

REM Clean up registry keys
REG DELETE "HKCU\Console\Text Adventure" /f