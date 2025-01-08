#!/bin/bash
#
# place wallpapers in C:/Users/<USERNAME>/Pictures/Wallpapers/ directorires

DEST=/mnt/c/Users/nkitan/Pictures/Wallpapers/
SRC=/home/nkitan/Work/autowall/wallpapers/

cp $SRC/* $DEST/
