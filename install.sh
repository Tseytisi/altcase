#!/bin/bash
# AltCase install script
# Tseytisi, May 2023

# This script performs the following actions, in order
# 1. Check if cargo is installed
# 2. Compile AltCase with Cargo
# 3. Moving the compiled binary to its location,
# and moving the required assets (such as images) to
# their folders too
# 4. If 'add_desktop_file' is true, create and copy
# a .desktop file to the correct folder

# Location where the executable needs to be stored (do not end this path with a '/')
binary_install_dir="/usr/bin"
# Location where the assets, such as images required by the GUI, are stored (do not end this path with a '/')
assets_install_dir="/usr/share/altcase"

# If true, adds a desktop file to /usr/share/applications/ which starts the application in GUI mode and makes it show up in application docks, start menu's, etc.
add_desktop_file=true

# 1. Checking for Cargo
if [ ! `command -v cargo` ]; then
    echo Cargo is not installed \- Please install it and rerun
    exit 1
fi

# 2. Compiling
echo Compiling altcase...
export ALTCASE_ASSETS_DIR="$assets_install_dir/"
if [ ! `cargo build --release --all-features` ]; then
    echo Successfully compiled
else
    echo Compiling failed
    exit 1
fi

# 3.
echo Moving files to their appropriate locations...
if [ ! `cp "target/release/altcase" "$binary_install_dir/" >> /dev/null 2>&1` ]; then
    echo Root access is required to copy a file to \'$binary_install_dir\'
    sudo cp "target/release/altcase" "$binary_install_dir/"
    sudo chmod a+rx "$binary_install_dir/altcase"
else
    chmod a+rx "$binary_install_dir/altcase"
fi
if [ ! -d "$assets_install_dir/img" ]; then
    if [ ! `mkdir -p "$assets_install_dir/img" >> /dev/null 2>&1` ]; then
        echo Root access is required to create the directory \'$assets_install_dir\'
        sudo mkdir -p "$assets_install_dir/img"
    fi
fi
if [ ! `cp "img/altcase.svg" "$assets_install_dir/img/" >> /dev/null 2>&1` ]; then
    echo Root access is required to copy to \'$assets_install_dir\'
    sudo cp "img/altcase.svg" "$assets_install_dir/img/"
    sudo chmod a+rx "$assets_install_dir/img/altcase.svg"
else
    chmod a+rx "$assets_install_dir/img/altcase.svg"
fi
if [ ! `cp "img/arrow.png" "$assets_install_dir/img/" >> /dev/null 2>&1` ]; then
    echo Root access is required to copy to \'$assets_install_dir\'
    sudo cp "img/arrow.png" "$assets_install_dir/img/"
    sudo chmod a+rx "$assets_install_dir/img/arrow.png"
else
    chmod a+rx "$assets_install_dir/img/arrow.png"
fi

echo Done

# 4.
desktop_file='[Desktop Entry]
Type=Application
Name=AltCase
Comment=Convert text to alternating case
Exec='$binary_install_dir'/altcase --gui
Icon='$assets_install_dir'/img/altcase.svg
Categories=Utility'

if [ $add_desktop_file ]; then
    echo Creating desktop file
    echo "$desktop_file" > /tmp/altcase.desktop
    if [ ! `cp "/tmp/altcase.desktop" "/usr/share/applications/" >> /dev/null 2>&1` ]; then
        echo Root access is required to copy a file to \'/usr/share/applications\'
        sudo cp "/tmp/altcase.desktop" "/usr/share/applications/"
    fi
fi

echo Installation completed
