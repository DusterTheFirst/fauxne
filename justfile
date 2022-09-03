PICO_BOOTSEL_MOUNT := "/run/media/dusterthefirst/RPI-RP2/"


default:
    @just --list

build:
    cmake --build {{justfile_directory()}}/build --config Debug --target all -j $(grep -c ^processor /proc/cpuinfo) --

upload: build
    sudo picotool reboot -f -u
    inotifywait /run/media/dusterthefirst/ -e create
    sleep 1
    cp {{justfile_directory()}}/build/fauxne.uf2 {{PICO_BOOTSEL_MOUNT}}

minicom:
    minicom -b 115200 -o -D /dev/ttyACM0 -c on

screen:
    screen /dev/ttyACM0

sleep:
    sleep 1
