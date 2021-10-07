.DEFAULT_GOAL = build

ARMGNU ?= aarch64-linux-gnu

BOOT_SRC_DIR = boot

RUST_TARGET = aarch64-unknown-none
RUST_BUILD_DIR = target/$(RUST_TARGET)/debug

clean:
	cargo clean

KERNEL8_ELF = $(RUST_BUILD_DIR)/kernel/kernel8.elf
KERNEL8_IMG = $(RUST_BUILD_DIR)/kernel/kernel8.img

$(KERNEL8_ELF):
	cargo build
	mkdir $(RUST_BUILD_DIR)/kernel
	cp $(RUST_BUILD_DIR)/rpi_os $(RUST_BUILD_DIR)/kernel/kernel8.elf

build $(KERNEL8_IMG): $(KERNEL8_ELF)
	rust-objcopy -O binary $(RUST_BUILD_DIR)/kernel/kernel8.elf $(RUST_BUILD_DIR)/kernel/kernel8.img

install-toolchain:
	rustup target add aarch64-unknown-none
	cargo install cargo-binutils -f
	rustup component add llvm-tools-preview

run: $(KERNEL8_IMG)
	qemu-system-aarch64 -M raspi3 -kernel $(KERNEL8_IMG) -serial stdio

.PHONY: install-toolchain run build
