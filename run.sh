#!/bin/bash

#doplhin root access!
#mount

sudo -s exit
read -rsp $'Press enter to continue...\n'
sudo losetup -P --show /dev/loop0 storage.img
sudo mount -ouser,umask=0000 /dev/loop0p1 _mnt
read -rsp $'create file!\n'
sudo umount /dev/loop0p1
sudo losetup -d /dev/loop0
cargo run
