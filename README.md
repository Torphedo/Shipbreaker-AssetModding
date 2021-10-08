# Shipbreaker Asset Mods
This repo hosts my mods for Hardspace: Shipbreaker that edit assets, such as textures. These are hosted as `*.emip` files, which can be created using [UABE](https://github.com/DerPopo/UABE). I've also included a program and a batch script (interchangable) I made to install these files, which I've made as easy to use and expandable as I can.

## Script/Executable Usage:
Download the batch script or executable from the releases tab, then place it in the Shipbreaker directory (next to `Shipbreaker.exe`). When run for the first time, it will download all the `*.emip` and `*.xdelta` mod files it needs directly from this repo, along with a copy of UABE and xdelta to apply them. When prompted, pick what mod you want to install, and the program will do it for you.

## Building the Executable Installer
The executable installer is based off of the batch script, and is written in Rust. Install [Rustup](https://www.rust-lang.org/tools/install), then run `build.bat`. An executable will appear next to the batch script.

You can also run `cargo build --release`, which will generate the executable at `.\target\release`.
