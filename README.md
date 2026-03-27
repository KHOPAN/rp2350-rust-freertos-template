# Overview
A FreeRTOS project template in C for RP2350 that integrates with Rust, with cross-language Link Time Optimization.

# Things to change
- Project names: line 41 of `CMakeLists.txt` and line 9 of `Cargo.toml`. (They must match)

# Building
Set the environment variable `ARM_TOOLCHAIN_PATH` to the Arm Toolchain for Embedded, which you can download [here](https://github.com/arm/arm-toolchain/releases). Also, make sure that the LLVM version matches your Rust toolchain. You can check it by running: `rustc --verbose --version`. Which should output something like this:
```
rustc 1.89.0 (29483883e 2025-08-04)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: x86_64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.7
```
Alternatively, you can edit line 39 of `CMakeLists.txt` to point to the toolchain directly. Then, you can build it like a normal Raspberry Pi Pico 2 project.

# How it works
It basically replaces the Pico-side compiler (from GCC) with Clang, which exposes us to the LLVM world. Then it adds a custom Cargo compile command to compile the Rust side, with Thin LTO enabled. So it can be combined with C, optimized during link time, and produce the final `uf2` binary.

# Provided example code
The provided example code creates a FreeRTOS task named `Main` from Rust. Since this template does not provide a FreeRTOS binding in Rust, you have to write it yourself. This template includes a simple Rust global memory allocator that allocates FreeRTOS memory.
