#!/usr/bin/env -S just --justfile

default:
    @just --list

build:
    cmake --build {{justfile_directory()}}/build --config Debug --target all -j $(grep -c ^processor /proc/cpuinfo) --

upload: build
    sudo picotool load -f -v {{justfile_directory()}}/build/fauxne.uf2

minicom:
    minicom -b 115200 -o -D /dev/ttyACM0 -c on

screen:
    screen /dev/ttyACM0

sleep:
    sleep 1
