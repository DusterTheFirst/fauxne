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

pico_set_program_name(fauxne "fauxne")
pico_set_program_version(fauxne "0.1")

pico_enable_stdio_uart(fauxne 1)
pico_enable_stdio_usb(fauxne 0)

target_link_libraries(fauxne
        hardware_spi
        hardware_i2c
        hardware_dma
        hardware_pio
        hardware_interp
        hardware_timer
        hardware_watchdog
        hardware_clocks
        pico_cyw43_arch_lwip_threadsafe_background
        pico_stdlib
)

pico_add_extra_outputs(fauxne)
