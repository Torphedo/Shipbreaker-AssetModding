@echo off
echo Building Shipbreaker Mod Installer...
echo.
if exist "shipbreaker-mod-installer.exe" (del shipbreaker-mod-installer.exe)
if exist "target\release\shipbreaker-mod-installer.exe" (del target\release\shipbreaker-mod-installer.exe)
if exist "mod_config.ini" (del "mod_config.ini")
cargo build --release
echo.
echo Build Finished.
if exist "E:\SteamLibrary\steamapps\common\Hardspace Shipbreaker\Shipbreaker-Mod-Installer.exe" (del "E:\SteamLibrary\steamapps\common\Hardspace Shipbreaker\Shipbreaker-Mod-Installer.exe")
copy target\release\shipbreaker-mod-installer.exe . > NUL
ren shipbreaker-mod-installer.exe Shipbreaker-Mod-Installer.exe > NUL
copy .\Shipbreaker-Mod-Installer.exe "E:\SteamLibrary\steamapps\common\Hardspace Shipbreaker" > NUL
pause