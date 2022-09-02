default:
    @just --list

build:
    cmake --build {{justfile_directory()}}/build --config RelWithDebInfo --target all -j $(grep -c ^processor /proc/cpuinfo) --

upload: build
    sudo picotool reboot -f -u
    inotifywait /run/media/dusterthefirst/ -e create
    sleep 1
    cmake --install {{justfile_directory()}}/build

minicom:
    minicom -b 115200 -o -D /dev/ttyACM0 -c on

screen:
    screen /dev/ttyACM0
