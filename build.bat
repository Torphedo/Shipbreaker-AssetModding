@echo off
echo Building Shipbreaker Mod Installer...
echo.
if exist "shipbreaker-mod-installer.exe" (del shipbreaker-mod-installer.exe)
if exist "target\release\shipbreaker-mod-installer.exe" (del target\release\shipbreaker-mod-installer.exe)
if exist "emip\" (rmdir /S /Q emip\)
if exist "mod_config.ini" (del "mod_config.ini")
cargo build --release
copy target\release\shipbreaker-mod-installer.exe .
pause