#!/bin/bash

sudo -s exit
read -rsp $'Press enter to continue...\n'
sudo losetup -P --show /dev/loop5 storage.img
sudo mount /dev/loop5p1 _mnt
read -rsp $'create file!\n'
sudo umount /dev/loop5p1
sudo losetup -d /dev/loop5
cargo run
