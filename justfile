alias help := _list

# (default) list the avaliable recipes
_list:
    @just --list --unsorted --justfile {{justfile()}}

# build, upload, and run the code with optimization level opt (default = 0)
run opt = "0": (upload opt)
    mpremote run main.py

# build and upload the code with optimization level opt (default = 0)
upload opt = "0": (build opt)
    mpremote fs cp -r static_files :
    cd build && mpremote fs cp -r . :

# build the code with optimization level opt (default = 0)
build opt = "0":
    #!/usr/bin/env sh
    mkdir -p build
    for file in `find src -type f -name "*.py"`; do
        outfile="build/$(echo $file | sed s/\.py/\.mpy/)"
        mkdir -p $(dirname $outfile)
        echo "Compiling $file"
        mpy-cross -o $outfile -O{{ opt }} -march=armv6m -v $file
    done

# clean the local build files
clean:
    #!/usr/bin/env sh
    if [ -e build ]; then
        rm -r build
        echo "cleaned"
    else
        echo "nothing to clean"
    fi

# wipe the attached pico
wipe:
    mpremote run helpers/wipe.py

# get information about the attached pico's mpy version
mpy:
    mpremote run helpers/mpy.py

# connect to the attached pico
connect:
    mpremote

# soft-reset the attached pico
reset:
    mpremote soft-reset
