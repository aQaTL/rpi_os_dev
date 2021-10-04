ARMGNU ?= aarch64-linux-gnu

BUILD_DIR = build
BOOT_SRC_DIR = boot

RUST_TARGET = aarch64-unknown-none
RUST_BUILD_DIR = target/$(RUST_TARGET)/debug

all: kernel8.img

clean:
	rm -rf $(BUILD_DIR)
	cargo clean

rust-build:
	cargo build

$(BUILD_DIR)/kernel8.img: $(BOOT_SRC_DIR)/linker.ld rust-build
	mkdir -p $(BUILD_DIR)
	$(ARMGNU)-ld \
		-T $(BOOT_SRC_DIR)/linker.ld \
		-o $(BUILD_DIR)/kernel8.elf \
		$(RUST_BUILD_DIR)/librpi_os.a
	$(ARMGNU)-objcopy $(BUILD_DIR)/kernel8.elf -O binary $(BUILD_DIR)/kernel8.img

install-toolchain:
	sudo apt install gcc-aarch64-linux-gnu -y
	rustup target add aarch64-unknown-none

run: $(BUILD_DIR)/kernel8.img
	qemu-system-aarch64 -M raspi3 -kernel $(BUILD_DIR)/kernel8.elf -serial stdio

.PHONY: install-toolchain run rust-build