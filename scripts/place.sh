#!/bin/bash
#
# place wallpapers in C:/Users/<USERNAME>/Pictures/Wallpapers/ directory

DEST=/mnt/c/Users/<USER>/Pictures/Wallpapers/
SRC=/opt/autowall/wallpapers/

cp $SRC/* $DEST/
