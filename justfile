list:
    @just --list --unsorted --justfile {{justfile()}}

run opt="0": (upload opt)
    mpremote run main.py

upload opt: (build opt)
    mpremote fs cp -r static_files :
    cd build && mpremote fs cp -r . :

wipe:
    mpremote run helpers/wipe.py

mpy:
    mpremote run helpers/mpy.py

connect:
    mpremote

reset:
    mpremote soft-reset

build opt:
    #!/usr/bin/env sh
    mkdir -p build
    for file in `find src -type f -name "*.py"`; do
        outfile="build/$(echo $file | sed s/\.py/\.mpy/)"
        mkdir -p $(dirname $outfile)
        echo "Compiling $file"
        mpy-cross -o $outfile -O{{ opt }} -march=armv6m -v $file
    done

clean:
    #!/usr/bin/env sh
    if [ -e build ]; then
        rm -r build
        echo "cleaned"
    else
        echo "nothing to clean"
    fi

