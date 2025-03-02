BUILD_DIR      := build
RUST_SRC       := src
ASM_SRC        := src_asm
C_SRC          := src_c

TARGET         := aarch64-unknown-none
KERNEL_NAME    := kernel
KERNEL_ELF     := target/$(TARGET)/release/$(KERNEL_NAME)
KERNEL_BIN     := binary/kernel8.img
LINKER_SCRIPT  := linker.ld

# Buscar recursivamente todos los archivos assembly y C.
ASM_SRCS       := $(shell find $(ASM_SRC) -name '*.S')
C_SRCS         := $(shell find $(C_SRC) -name '*.c')

# Generar la ruta de objetos, preservando la estructura relativa.
ASM_OBJS       := $(patsubst $(ASM_SRC)/%.S, $(BUILD_DIR)/%.o, $(ASM_SRCS))
C_OBJS         := $(patsubst $(C_SRC)/%.c, $(BUILD_DIR)/%.o, $(C_SRCS))

OBJS           := $(ASM_OBJS) $(C_OBJS)

AS             := aarch64-none-elf-as
CC             := aarch64-none-elf-gcc
OBJCOPY        := rust-objcopy
CARGO          := cargo

# Se agregan los objetos como argumentos de linkeo.
RUSTFLAGS_EXTRA := $(foreach obj, $(OBJS), -C link-arg=$(obj))

.PHONY: all preclean clean

# Al invocar "make", se ejecuta primero preclean que elimina build, target y binary.
all: preclean $(KERNEL_BIN)

preclean:
	rm -rf $(BUILD_DIR) target binary
	mkdir binary

# Regla para crear el directorio build (aunque cada regla también crea su directorio intermedio)
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Regla genérica para compilar archivos assembly (.S) recursivamente.
$(BUILD_DIR)/%.o: $(ASM_SRC)/%.S
	@mkdir -p $(dir $@)
	$(CC) -c -x assembler-with-cpp -Iinclude $< -o $@

# Regla genérica para compilar archivos C (.c) recursivamente.
$(BUILD_DIR)/%.o: $(C_SRC)/%.c
	@mkdir -p $(dir $@)
	$(CC) -c -Iinclude/** $< -o $@

# Compilar el proyecto Rust en modo release, pasando los objetos extra.
$(KERNEL_ELF): $(OBJS)
	$(CARGO) rustc --release --target=$(TARGET) -- $(RUSTFLAGS_EXTRA)

# Generar la imagen binaria final a partir del ELF.
$(KERNEL_BIN): $(KERNEL_ELF)
	$(OBJCOPY) --strip-all -O binary $< $@

