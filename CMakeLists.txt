cmake_minimum_required(VERSION 3.13)
set(CMAKE_C_STANDARD 11)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(FREERTOS_CONFIG_FILE_DIRECTORY ${CMAKE_SOURCE_DIR}/include)
set(FREERTOS_KERNEL_PATH ${CMAKE_SOURCE_DIR}/dependencies/FreeRTOS-Kernel)
set(PICO_BOARD pico2_w)
set(PICO_COMPILER pico_arm_cortex_m33_clang)
set(PICO_PLATFORM rp2350)
set(PROJECT_C_SOURCE_FILES
	src/c/main.c
)

set(PROJECT_RUST_SOURCE_FILES
	src/rust/freertos.rs
	src/rust/main.rs
)

set(PROJECT_DEPENDENCIES
	FreeRTOS-Kernel
	FreeRTOS-Kernel-Heap4
	pico_stdlib
)

# == DO NOT EDIT THE FOLLOWING LINES for the Raspberry Pi Pico VS Code Extension to work ==
if(WIN32)
	set(USERHOME $ENV{USERPROFILE})
else()
	set(USERHOME $ENV{HOME})
endif()
set(sdkVersion 2.2.0)
set(toolchainVersion 14_2_Rel1)
set(picotoolVersion 2.2.0)
set(picoVscode ${USERHOME}/.pico-sdk/cmake/pico-vscode.cmake)
if (EXISTS ${picoVscode})
	include(${picoVscode})
endif()
# =========================================================================================

set(PICO_TOOLCHAIN_PATH $ENV{ARM_TOOLCHAIN_PATH})
include(${CMAKE_SOURCE_DIR}/pico_sdk_import.cmake)
project(rp2350_rust_freertos_template C CXX ASM)
pico_sdk_init()
include(${CMAKE_SOURCE_DIR}/dependencies/FreeRTOS-Kernel/portable/ThirdParty/GCC/RP2350_ARM_NTZ/FreeRTOS_Kernel_import.cmake)
set(RUST_LIBRARY ${CMAKE_SOURCE_DIR}/target/thumbv8m.main-none-eabihf/release/lib${PROJECT_NAME}.a)
add_custom_command(
	OUTPUT ${RUST_LIBRARY}
	COMMAND cargo build --release
	DEPENDS ${CMAKE_SOURCE_DIR}/.cargo/config.toml ${CMAKE_SOURCE_DIR}/Cargo.toml ${CMAKE_SOURCE_DIR}/rust-toolchain.toml ${PROJECT_RUST_SOURCE_FILES}
	WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
	COMMENT "Compiling Rust sources..."
)

add_executable(${PROJECT_NAME} ${PROJECT_C_SOURCE_FILES})
add_custom_target(rust_library_target DEPENDS ${RUST_LIBRARY})
add_library(rust_library STATIC IMPORTED)
pico_enable_stdio_uart(${PROJECT_NAME} 1)
pico_enable_stdio_usb(${PROJECT_NAME} 1)
set_target_properties(rust_library PROPERTIES IMPORTED_LOCATION ${RUST_LIBRARY})
target_compile_options(${PROJECT_NAME} PRIVATE -flto=thin -mfloat-abi=hard -mfpu=fpv5-sp-d16 -O3)
target_link_libraries(${PROJECT_NAME} rust_library ${PROJECT_DEPENDENCIES})
target_link_options(${PROJECT_NAME} PRIVATE -flto=thin -fuse-ld=lld -mfloat-abi=hard -mfpu=fpv5-sp-d16 ${RUST_LIBRARY})
pico_add_extra_outputs(${PROJECT_NAME})
