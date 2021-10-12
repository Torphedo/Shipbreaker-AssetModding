@echo off
color 06
:setup
if exist "emip\check.version5" goto :start
if exist emip\ (rmdir /S /Q emip\)
mkdir emip
echo Downloading mod files...
set file=Jupiter.emip
call :curl_bin
set file=Moon.emip
call :curl_bin
set file=NoBloom.emip
call :curl_bin
set file=AlphaTitleScreen.emip
call :curl_bin
set file=ModdingSticker.emip
call :curl_bin
set file=BBI.Unity.Game.xdelta
call :curl_bin
set file=Carbon.Core.xdelta
call :curl_bin
set file=check.version5
call :curl_bin
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
cd emip\
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
echo      7. Maps
echo      8. Live Debugger (UnityExplorer)
echo      9. Exit
echo.
echo.
set /P selection="Select an option: "
cls
if %selection%==9 (exit)
if %selection%==1 (set emip=Jupiter.emip&goto :apply_emip)
if %selection%==2 (set emip=Moon.emip&goto :apply_emip)
if %selection%==3 (set emip=NoBloom.emip&goto :apply_emip)
if %selection%==4 (set emip=AlphaTitleScreen.emip&goto :apply_emip)
if %selection%==5 (
	set DLL=BBI.Unity.Game
	call :delta_patch
	set DLL=Carbon.Core
	call :delta_patch
	curl -# -o ..\mod_config.ini https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/mod_config.ini
	echo Patch Applied.
	pause
	goto :start
)
if %selection%==6 (set emip=ModdingSticker.emip&goto :apply_emip)
if %selection%==8 (
	echo Downloading BepInEx...
	curl -L -# -o ..\BepInEx.zip https://github.com/BepInEx/BepInEx/releases/download/v5.4.17/BepInEx_x64_5.4.17.0.zip
	echo Downloading UnityExplorer...
	curl -L -# -o ..\UnityExplorer.zip https://github.com/sinai-dev/UnityExplorer/releases/download/4.3.2/UnityExplorer.BepInEx5.Mono.zip
	echo Extracting...
	powershell -command "Expand-Archive ..\BepInEx.zip .."
	mkdir BepInEx\plugins
	powershell -command "Expand-Archive ..\UnityExplorer.zip ..\BepInEx\plugins"
	del ..\BepInEx.zip
	del ..\UnityExplorer.zip
	del ..\changelog.txt
	echo Installation Complete.
	pause
	goto :start
)
if %selection%==7 (
	:map_options
	copy Shipbreaker_Data\level6 Shipbreaker_Data\level14
	echo When you select a map from this list, it will be loaded instead of the cutting bay.
	echo To revert to the normal map, verify your game files, or select the same map again.
	echo.
	echo.
	echo       1. Master Jack (just the Master Jack, a ship, and infinite void)
	echo       2. Elemental Test Map (fun object spawners, replaces Master Jack)
	echo       3. Subsystem Test Map (replaces Master Jack)
	echo       4. QA map (TONS of super old scrapped concepts from early in development)
	echo       5. Deep Space
	echo       6. Back
	echo.
	echo.
	set /P map="Select a map: "
	if %map%==1 (set num1=8&set num2=6)
	if %map%==2 (set num1=9&set num2=7)
	if %map%==3 (set num1=10&set num2=9)
	if %map%==4 (set num1=5&set num2=8)
	if %map%==5 (set num1=14&set num2=9&call :scene_swap&set num1=8&set num2=6)
	if %map%==6 (goto :start)
	call :scene_swap
	pause
	cls
	goto :map_options
)

:delta_patch
xdelta -d -s ..\Shipbreaker_Data\Managed\%DLL%.dll %DLL%.xdelta ..\Shipbreaker_Data\Managed\%DLL%.dll.mod
del ..\Shipbreaker_Data\Managed\%DLL%.dll
ren ..\Shipbreaker_Data\Managed\%DLL%.dll.mod %DLL%.dll
exit /b

:scene_swap
echo Swapping Unity scene files...
ren ..\Shipbreaker_Data\level%num1% level%num1%.temp
if exist ..\Shipbreaker_Data\level%num1%.resS (ren ..\Shipbreaker_Data\level%num1%.resS level%num1%.resS.temp)
ren ..\Shipbreaker_Data\level%num2% level%num1%
if exist ..\Shipbreaker_Data\level%num2%.resS (ren ..\Shipbreaker_Data\level%num2%.resS level%num1%.resS)
ren ..\Shipbreaker_Data\level%num1%.temp level%num2%
if exist ..\Shipbreaker_Data\level%num1%.resS.temp (ren ..\Shipbreaker_Data\level%num1%.resS.temp level%num2%.resS)
echo Done.
exit /b

:apply_emip
UABE\AssetBundleExtractor applyemip %emip% ..
echo Patch applied.
pause
goto :start