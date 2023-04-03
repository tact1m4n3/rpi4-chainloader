TARGET = aarch64-unknown-none-softfloat

LOADER_BIN = kernel8.img
LOADER_ELF = target/$(TARGET)/release/loader

CARGO_ARGS = --package loader --release --target $(TARGET)
CARGO_CMD = cargo build $(CARGO_ARGS)

OBJCOPY_ARGS = --strip-all -O binary
OBJCOPY_CMD = rust-objcopy $(OBJCOPY_ARGS)

OBJDUMP_ARGS = --disassemble --demangle
OBJDUMP_CMD = rust-objdump $(OBJDUMP_ARGS)

.PHONY: all clean loader_elf objdump

all: $(LOADER_BIN)

loader_elf:
	$(CARGO_CMD)

$(LOADER_BIN): loader_elf
	$(OBJCOPY_CMD) $(LOADER_ELF) $(LOADER_BIN)

objdump: loader_elf
	$(OBJDUMP_CMD) $(LOADER_ELF)

clean:
	cargo clean
	rm -rf $(KERNEL_BIN)
