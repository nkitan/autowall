#!/bin/bash
#
# place wallpapers in C:/Users/<USERNAME>/Pictures/Wallpapers/ directorires

DEST=/mnt/c/Users/nkitan/Pictures/Wallpapers/
SRC=/opt/autowall/wallpapers/

cp $SRC/* $DEST/
