# Generated Cmake Pico project file

cmake_minimum_required(VERSION 3.13)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

set(PICO_BOARD pico_w)

# Pull in Raspberry Pi Pico SDK (must be before project)
include(pico_sdk_import.cmake)

project(fauxne C CXX ASM)

# Initialise the Raspberry Pi Pico SDK
pico_sdk_init()

if(COMMAND suppress_tinyusb_warnings)
    # Explicitly suppress warnings in TinyUSB files which have them (this has to be done
    # from the project that uses them per CMake "feature"). Note the function comes from
    # TinyUSB itself, so we have to guard against TinyUSB not being present with the above if
    suppress_tinyusb_warnings()
endif()

if(TARGET pico_lwip)
    set_source_files_properties(
        ${PICO_LWIP_PATH}/src/core/ipv4/ip4_frag.c
        PROPERTIES
        COMPILE_FLAGS "-Wno-null-dereference")
endif()

# DHCP server
add_library(dhcpserver STATIC libs/vendor/dhcpserver/dhcpserver.c)
target_include_directories(dhcpserver SYSTEM INTERFACE libs/vendor/dhcpserver)
target_include_directories(dhcpserver PRIVATE lwip)
target_link_libraries(dhcpserver
    pico_cyw43_arch_lwip_threadsafe_background
)

# HTTP parser
add_library(llhttp STATIC
    libs/vendor/llhttp/src/llhttp.c
    libs/vendor/llhttp/src/http.c
    libs/vendor/llhttp/src/api.c)
target_include_directories(llhttp SYSTEM INTERFACE libs/vendor/llhttp/include)
target_include_directories(llhttp PRIVATE libs/vendor/llhttp/include)

# logging facade strings
add_library(log INTERFACE)
target_include_directories(log INTERFACE libs/log/include)
target_link_libraries(log INTERFACE pico_stdlib)

# Rust-style strings
add_library(str INTERFACE)
target_include_directories(str INTERFACE libs/str/include)
target_link_libraries(str INTERFACE log)

# Static files include
file(GLOB STATIC_FILES ${CMAKE_CURRENT_SOURCE_DIR}/static_files/*)
add_custom_command(OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/generated/static_files/static_files.c
    DEPENDS ${STATIC_FILES} ${CMAKE_CURRENT_SOURCE_DIR}/generate_static.sh
    COMMAND ${CMAKE_CURRENT_SOURCE_DIR}/generate_static.sh
    VERBATIM
)

add_library(static_files STATIC ${CMAKE_CURRENT_BINARY_DIR}/generated/static_files/static_files.c)
target_link_libraries(static_files str)

target_include_directories(static_files
    INTERFACE ${CMAKE_CURRENT_BINARY_DIR}/generated/static_files/include
)

# Get all of the c files in the src directory
file(GLOB SOURCES src/*.c src/*/*.c)

add_executable(fauxne ${SOURCES})

# Add the headers
target_include_directories(fauxne PRIVATE
    include
    lwip
)

pico_enable_stdio_uart(fauxne 1)
pico_enable_stdio_usb(fauxne 1)

pico_add_extra_outputs(fauxne)

target_link_libraries(fauxne
    hardware_interp
    pico_cyw43_arch_lwip_threadsafe_background
    pico_stdlib
    pico_multicore
    pico_time
    dhcpserver
    llhttp
    static_files
    str
    log
)

target_compile_definitions(fauxne PRIVATE

    # PICO_STDIO_ENABLE_CRLF_SUPPORT=1
    LOG_LEVEL=LOG_LEVEL_TRACE
    PICO_STDIO_USB_STDOUT_TIMEOUT_US=2000000000
    PICO_DEFAULT_UART_BAUD_RATE=9600

    WAIT_FOR_USB_CDC=1

    # PICO_PANIC_FUNCTION=pico_panic
)

target_compile_options(fauxne PRIVATE

    # Remove absoulte path from __FILE__
    # see https://stackoverflow.com/a/53848526
    -ffile-prefix-map=${CMAKE_SOURCE_DIR}/=

    -Werror
    -Wall
    -Wextra

    # -pedantic
    -Wnull-dereference
    -Wuninitialized
    -Wunused
    -Wcast-align
    -Wall
    -Wcast-qual
    -Wfloat-equal
    -Wmissing-format-attribute

    # -Wconversion
    -Wsign-compare
    $<$<COMPILE_LANGUAGE:C>:-Wstrict-prototypes>

    -Wno-inline

    # todo not sure these are true, but investigate
    # -Wpacked

    # todo we have some of these in usb_device_tiny to try to make it more readable.. perhaps doxygen would help here instead
    -Wredundant-decls

    -Wno-shadow
    -Wno-missing-field-initializers
    -Wno-missing-braces
    -Wno-multichar

    # todo useful but fix later
    # -Wundef
)
