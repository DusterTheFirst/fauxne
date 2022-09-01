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

# Get all of the c files in the src directory
file(GLOB SOURCES src/*.c)

add_executable(fauxne ${SOURCES})

# Add the headers
target_include_directories(fauxne PRIVATE include)
target_include_directories(fauxne SYSTEM INTERFACE include)

target_compile_options(fauxne PRIVATE
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

pico_set_program_name(fauxne "fauxne")
pico_set_program_version(fauxne "0.1")

pico_enable_stdio_uart(fauxne 1)
pico_enable_stdio_usb(fauxne 0)

target_link_libraries(fauxne
        hardware_interp
        hardware_timer
        pico_cyw43_arch_lwip_threadsafe_background
        pico_stdlib
)

add_compile_definitions(
        # PICO_STDIO_ENABLE_CRLF_SUPPORT=0
        LOG_LEVEL=LOG_LEVEL_TRACE
)

pico_add_extra_outputs(fauxne)
