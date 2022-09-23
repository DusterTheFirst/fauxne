use std::{
    env,
    io::{self, Cursor},
    path::PathBuf,
    process::{exit, Command, ExitCode},
};

fn main() {
    let pico_sdk_path = env::var("PICO_SDK_PATH").expect("PICO_SDK PATH defined");

    let include_directories = Command::new("find")
        .arg(format!("{pico_sdk_path}/src/common"))
        .arg(format!("{pico_sdk_path}/src/rp2_common"))
        .args(["-type", "d"])
        .args(["-name", "include"])
        .output()
        .expect("failed to run `find` command");

    if !include_directories.status.success() {
        io::copy(
            &mut Cursor::new(include_directories.stderr),
            &mut io::stderr(),
        )
        .expect("copies command stderr to stderr");
        io::copy(
            &mut Cursor::new(include_directories.stdout),
            &mut io::stderr(),
        )
        .expect("copies command stderr to stderr");

        exit(include_directories.status.code().unwrap_or(-1));
    }

    let include_directories = String::from_utf8(include_directories.stdout)
        .expect("`find` command produced non UTF-8 output");

    let mut includes_arg = Vec::new();
    let mut wrapper_header = String::new();

    for directory in include_directories.trim().split('\n') {
        includes_arg.extend_from_slice(&["-I", directory]);

        let headers = Command::new("find")
            .arg(directory)
            .args(["-type", "f"])
            .args(["-printf", "%P\n"])
            .output()
            .expect("failed to run `find` command");

        if !headers.status.success() {
            io::copy(&mut Cursor::new(headers.stderr), &mut io::stderr())
                .expect("copies command stderr to stderr");
            io::copy(&mut Cursor::new(headers.stdout), &mut io::stderr())
                .expect("copies command stderr to stderr");

            exit(headers.status.code().unwrap_or(-1));
        }

        let headers =
            String::from_utf8(headers.stdout).expect("`find` command produced non UTF-8 output");

        for header in headers.trim().split('\n') {
            wrapper_header.push_str("#include \"");
            wrapper_header.push_str(header);
            wrapper_header.push_str("\"\n");
        }
    }

    let bindings = bindgen::builder()
        .use_core()
        .generate_inline_functions(true)
        // .prepend_enum_name(false)
        .header_contents("wrapper.h", &wrapper_header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(["-D", "PICO_PLATFORM=rp2040"])
        .clang_args(includes_arg)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// find $PICO_SDK_PATH/src -type d -name include

/*

bindgen wrapper.h \
    --use-core \
    --generate-inline-functions \
    --ctypes-prefix "crate::ctypes" \
    --disable-untagged-union \
    --no-prepend-enum-name \
    --no-layout-tests \
    -- \
    -I $PICO_SDK_PATH/src/rp2_common/pico_stdio/include \
    -I $PICO_SDK_PATH/src/common/pico_stdlib/include \
    -I $PICO_SDK_PATH/src/common/pico_base/include \
    -I $PICO_SDK_PATH/src/common/pico_time/include \
    -I $PICO_SDK_PATH/src/rp2_common/pico_platform/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_base/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_timer/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_gpio/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_uart/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_irq/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_pwm/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_spi/include \
    -I $PICO_SDK_PATH/src/rp2040/hardware_regs/include \
    -I $PICO_SDK_PATH/src/rp2040/hardware_structs/include \
    -I $PICO_SDK_PATH/src/boards/include \
    -I ./generated
*/
