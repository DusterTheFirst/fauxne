#!/bin/sh

set -euo pipefail

BASEDIR=$(dirname "$0")

static_dir="$BASEDIR/static_files"
static_out_dir="$BASEDIR/build/generated/static_files"
static_out_include="$static_out_dir/include"
static_out_header="$static_out_include/static_files.h"

autogenerate_header="
#include \"str.h\"

// ---------------------------------------
// THIS FILE IS AUTOGENERATED; DO NOT EDIT
// ---------------------------------------
";

mkdir --parents $static_out_include
echo -e "#pragma once\n$autogenerate_header" > $static_out_header


for file in $(ls $static_dir); do
    variable_name=$(echo "static_file_$file" | sed s/\\./_/g -)
    in_file="$static_dir/$file" 
    out_file_hex="$static_out_dir/$file.hex"
    out_file_c="$static_out_dir/$file.c"
    
    xxd -p "$in_file" "$out_file_hex"
    echo -e "$autogenerate_header\nconst static uint8_t $variable_name[] = {" > $out_file_c
    sed -r "s/(.{2})/0x\\1\\,/g" "$out_file_hex" >> $out_file_c
    echo "};
const str_t ${variable_name}_str = {
    .ptr = ${variable_name},
    .len = sizeof(${variable_name}),
};" >> $out_file_c

    echo "extern const str_t ${variable_name}_str;" >> $static_out_header
done
