# Bare-metal OS for Raspberry Pi Zero 2 W

This is a minimal operating system developed from scratch for the Raspberry Pi Zero 2 W (ARMv8-A architecture), without relying on existing operating systems or any kind of emulation.

![UART terminal output](example.gif)

## Overview

The goal of this project was to explore low-level programming on ARM hardware. It handles the boot process, basic GPIO control, and UART communication using memory-mapped I/O registers. Everything was tested directly on the device via a USB-to-UART connection.

## Features

- Bootloader written in ARM assembly
- GPIO pin control
- UART communication via PL011
- UART receive interrupts
- Basic interactive UART terminal (currently supports just a few demo commands: `test`, `test2`, `clear`/`cls`)
- Fully tested on hardware (no QEMU or emulators)
- Built in three iterations:
  - [First version (C++/ASM)](https://github.com/UNIX-73/RASPBERRY_PI_ZERO_2_W_BAREMETAL)
  - [Second version (C++/ASM)](https://github.com/UNIX-73/BAREMETAL_V2)
  - Current version (Rust/ASM, rewritten from scratch with improvements)

## Toolchain and Environment

- Cross-compilation: `aarch64-unknown-none`, `cargo`, `make`
- Languages: Assembly, Rust, C++ (the previous iterations)
- UART output via USB serial adapter
- Target: Raspberry Pi Zero 2 W

## How to Use

1. Copy the contents of the `rpi_boot/` folder to the root of your Raspberry Pi SD card.
2. Compile the kernel using `make` (or use the provided `kernel8.img` in the `binary/` folder).
3. Place or replace the resulting kernel8.img in the SD card root (alongside the files from rpi_boot/).
4. Connect the Pi to your computer via UART (minicom reccomended) and power it up. You’ll see output in the terminal.

## Notes

- This is an educational project and does not include features like MMU support, file systems, or HDMI output.
- Documentation is still in progress. If you are reviewing this and need more technical details, feel free to contact me.
