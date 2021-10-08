@echo off

:setup
if exist "emip\check.version4" goto :start
if exist emip\ (rmdir /S /Q emip\)
mkdir emip
echo Downloading mod files...
set file="Jupiter.emip"
call :curl_bin
set file="Moon.emip"
call :curl_bin
set file="NoBloom.emip"
call :curl_bin
set file="AlphaTitleScreen.emip"
call :curl_bin
set file="ModdingSticker.emip"
call :curl_bin
set file="Cheats.xdelta"
call :curl_bin
set file="Carbon.Core.xdelta"
call :curl_bin
set file="check.version4"
call :curl_bin
curl -# -o mod_config.ini https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/mod_config.ini

echo Downloading tools...
curl -L -# -o emip\UABE.zip https://github.com/DerPopo/UABE/releases/download/2.2stabled/AssetsBundleExtractor_2.2stabled_64bit.zip
curl -s -o emip\xdelta.exe https://raw.githubusercontent.com/marco-calautti/DeltaPatcher/master/xdelta.exe
powershell -command "Expand-Archive emip\UABE.zip emip"
ren emip\64bit UABE
del emip\UABE.zip
goto :start

:curl_bin
curl -# -o emip\%file% https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/%file%
exit /b

:start
cls
echo Welcome to Torph's Shipbreaker mod installer! If you've used
echo mods before, please verify your game files in Steam before
echo using, to avoid potential crashes.
echo.
echo.
echo      1. Jupiter over Earth
echo      2. Moon over Earth
echo      3. No Bloom
echo      4. Alpha Title Screen
echo      5. Cheats
echo      6. Modding Sticker
echo      7. Exit
echo.
echo.
set /P selection="Select a mod to install: "
cls
cd emip\

IF %selection%==1 (set emip=Jupiter.emip & goto :apply_emip)
IF %selection%==2 (set emip=Moon.emip & goto :apply_emip)
IF %selection%==3 (set emip=NoBloom.emip & goto :apply_emip)
IF %selection%==4 (set emip=AlphaTitleScreen.emip & goto :apply_emip)
IF %selection%==5 (
	set DLL=BBI.Unity.Game.dll
	set patchname=Cheats
	call :delta_patch
	set DLL=Carbon.Core.dll
	set patchname=Carbon.Core
	call :delta_patch
	echo Patch Applied.
	goto :post_install
)
IF %selection%==6 (set emip=ModdingSticker.emip & goto :apply_emip)

:delta_patch
xdelta -d -s ..\Shipbreaker_Data\Managed\%DLL% %patchname%.xdelta ..\Shipbreaker_Data\Managed\%DLL%.mod
del ..\Shipbreaker_Data\Managed\%DLL%
ren ..\Shipbreaker_Data\Managed\%DLL%.mod %DLL%
exit /b

:apply_emip
UABE\AssetBundleExtractor applyemip %emip% ..
echo Patch applied.

:post_install
set /P return="Return to list of mods? (y/n) "
IF %return%==y (cd .. & goto :start)