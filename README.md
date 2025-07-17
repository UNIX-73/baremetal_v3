# Bare-metal OS for Raspberry Pi Zero 2 W

This is a minimal operating system developed from scratch for the Raspberry Pi Zero 2 W (ARMv8-A architecture), without relying on any existing OS or emulation layers.

![UART terminal output](example.gif)

## Overview

The goal of this project was to gain hands-on experience with low-level programming on ARM hardware. It covers the full boot process, GPIO control, and UART communication using memory-mapped I/O registers. All testing was done directly on hardware via USB-to-UART connection.

## Features

- Bootloader written in ARMv8 assembly
- Direct control over GPIO pins
- PL011 UART communication
- UART receive interrupts
- Fully hardware-tested (no emulators, no QEMU)
- Exception Level change to EL1
- Developed over three iterations:
  - [Version 1 – C++/Assembly](https://github.com/UNIX-73/RASPBERRY_PI_ZERO_2_W_BAREMETAL)
  - [Version 2 – C++/Assembly](https://github.com/UNIX-73/BAREMETAL_V2)
  - **Version 3 (this repo)** – Rust + Assembly rewrite with improved structure and terminal features

## Interactive UART Terminal

The system includes a simple UART terminal implemented entirely on the Raspberry Pi. It maintains an internal input buffer, handles backspace, and parses input commands character by character, everything processed by the Pi itself.

### Supported commands:

- `test [arg1 arg2 ...]` → Prints `ran test` and echoes any provided arguments.
- `test2` → Prints `ran test2`. Takes no arguments.
- `clear` / `cls` → Clears the screen.

All output is sent over the UART connection and visible through a serial terminal (e.g., `minicom`).

## Toolchain and Environment

- Cross-compilation: `aarch64-unknown-none`, `cargo`, `make`
- Languages: Assembly (boot), Rust (kernel), C++ (previous versions)
- UART via USB serial adapter
- Target board: Raspberry Pi Zero 2 W

## How to Use

1. Copy the contents of the `rpi_boot/` folder to the root of your Raspberry Pi SD card.
2. Compile the kernel using `make`, or use the prebuilt `kernel8.img` from the `binary/` folder.
3. Place the `kernel8.img` in the root of the SD card.
4. Connect the Pi to your computer via UART and power it up. The terminal will display boot logs and accept commands.

## Notes

- This project is educational and focused on learning system-level development.
- It does not include features like MMU, multitasking, file systems, or HDMI support.
- If you’re reviewing this and would like further technical insight, feel free to reach out.
