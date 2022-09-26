list:
    @just --list --unsorted --justfile {{justfile()}}

run: upload build
    mpremote run main.py

upload: build
    mpremote fs cp -r static_files :
    cd build && mpremote fs cp -r . :

wipe:
    mpremote run helpers/wipe.py

connect:
    mpremote

reset:
    mpremote soft-reset

build:
    #!/usr/bin/env sh
    mkdir -p build
    for file in `find src -type f -name "*.py"`; do
        outfile="build/$(echo $file | sed s/\.py/\.mpy/)"
        mkdir -p $(dirname $outfile)
        echo "Compiling $file"
        mpy-cross -o $outfile -O9 -v $file
    done

clean:
    #!/usr/bin/env sh
    if [ -e build ]; then
        rm -r build
        echo "cleaned"
    else
        echo "nothing to clean"
    fi

