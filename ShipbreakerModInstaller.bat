@echo off

:setup
if exist "emip\Jupiter.emip" goto :start
mkdir emip
echo Downloading mod files...
curl -# -o emip\Jupiter.emip https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Jupiter.emip
curl -# -o emip\Moon.emip https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Moon.emip
curl -# -o emip\NoBloom.emip https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/NoBloom.emip
curl -# -o emip\AlphaTitleScreen.emip https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/AlphaTitleScreen.emip
curl -# -o emip\ModdingSticker.emip https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/ModdingSticker.emip
curl -# -o emip\Cheats.xdelta https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Cheats.xdelta
curl -# -o emip\Carbon.Core.xdelta https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Carbon.Core.xdelta
echo Downloading tools...
curl -L -# -o emip\UABE.zip https://github.com/DerPopo/UABE/releases/download/2.2stabled/AssetsBundleExtractor_2.2stabled_64bit.zip
curl -s -o emip\xdelta.exe https://raw.githubusercontent.com/marco-calautti/DeltaPatcher/master/xdelta.exe
powershell -command "Expand-Archive emip\UABE.zip emip"
ren emip\64bit UABE
del emip\UABE.zip

:start
echo Welcome to Torph's Shipbreaker mod installer script. If you've
echo used mods before, please verify your game files in Steam before
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
set /P ACTION="Enter the number of your selection:"
cls
cd emip\
goto :case_%ACTION%

:case_1
set emip=Jupiter.emip
goto :apply_emip

:case_2
set emip=Moon.emip
goto :apply_emip

:case_3
set emip=NoBloom.emip
goto :apply_emip

:case_4
set emip=AlphaTitleScreen.emip
goto :apply_emip

:case_5
xdelta -d -s ..\Shipbreaker_Data\Managed\BBI.Unity.Game.dll Cheats.xdelta ..\Shipbreaker_Data\Managed\BBI.Unity.Game.dll.mod
del ..\Shipbreaker_Data\Managed\BBI.Unity.Game.dll
ren ..\Shipbreaker_Data\Managed\BBI.Unity.Game.dll.mod BBI.Unity.Game.dll
xdelta -d -s ..\Shipbreaker_Data\Managed\Carbon.Core.dll Carbon.Core.xdelta ..\Shipbreaker_Data\Managed\Carbon.Core.dll.mod
del ..\Shipbreaker_Data\Managed\Carbon.Core.dll
ren ..\Shipbreaker_Data\Managed\Carbon.Core.dll.mod Carbon.Core.dll
echo Patch Applied.
goto :post_install

:case_6
set emip=ModdingSticker.emip
goto :apply_emip

:apply_emip
UABE\AssetBundleExtractor applyemip %emip% ..
echo Patch applied.

:post_install
set /P ACTION="Exit script? (y/n)"
cls
goto :post_%ACTION%

:post_n
cd ..
goto :start