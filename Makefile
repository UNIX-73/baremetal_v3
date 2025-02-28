BUILD_DIR      := build
RUST_SRC       := src
ASM_SRC        := src_boot

# Variables de configuración
TARGET         := aarch64-unknown-none
KERNEL_NAME    := kernel
KERNEL_ELF     := target/$(TARGET)/release/$(KERNEL_NAME)
KERNEL_BIN     := kernel8.img
LINKER_SCRIPT  := linker.ld
BOOT_ASM       := $(ASM_SRC)/boot.S
BOOT_OBJ       := $(BUILD_DIR)/boot.o

# Comandos (ajusta estos si tus herramientas tienen otro nombre o ruta)
AS             := aarch64-none-elf-as
OBJCOPY        := rust-objcopy
CARGO          := cargo

# Los flags de Rust (además de los ya definidos en .cargo/config.toml)
# Se añade el objeto boot.o al proceso de linkeo.
RUSTFLAGS_EXTRA := -C link-arg=$(BOOT_OBJ)

.PHONY: all clean

all: $(KERNEL_BIN)

# Compilar el ensamblador a objeto.
$(BOOT_OBJ): $(BOOT_ASM)
	$(AS) -o $@ $<

# Compilar el proyecto Rust en modo release.
# Se asume que en .cargo/config.toml ya está configurado el linker (con el linker script).
$(KERNEL_ELF): $(BOOT_OBJ)
	$(CARGO) rustc --release --target=$(TARGET) -- $(RUSTFLAGS_EXTRA)

# Generar la imagen binaria final a partir del ELF.
$(KERNEL_BIN): $(KERNEL_ELF)
	$(OBJCOPY) --strip-all -O binary $< $@

clean:
	$(CARGO) clean
	rm -f $(BOOT_OBJ) $(KERNEL_BIN)