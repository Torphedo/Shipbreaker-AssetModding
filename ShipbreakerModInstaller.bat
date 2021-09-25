@echo off

:setup
if exist "emip\Jupiter.emip" goto :start
mkdir emip
echo Downloading mod files...
curl -# -o emip\Jupiter.emip https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Jupiter.emip
curl -# -o emip\Moon.emip https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Moon.emip
curl -# -o emip\NoBloom.emip https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/NoBloom.emip
curl -# -o emip\AlphaTitleScreen.emip https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/AlphaTitleScreen.emip
echo Downloading tools...
curl -L -# -o emip\UABE.zip https://github.com/DerPopo/UABE/releases/download/2.2stabled/AssetsBundleExtractor_2.2stabled_64bit.zip
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
echo      5. Exit
echo.
echo.
set /P ACTION="Enter the number of your selection:"
cls
goto :case_%ACTION%

:case_1
cd emip\
set emip=Jupiter.emip
echo Patch applied.
goto :apply_emip

:case_2
cd emip\
set emip=Moon.emip
echo Patch applied.
goto :apply_emip

:case_3
cd emip\
set emip=NoBloom.emip
echo Patch applied.
goto :apply_emip

:case_4
cd emip\
set emip=AlphaTitleScreen.emip
echo Patch applied.
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