ARMGNU ?= aarch64-linux-gnu

BUILD_DIR = build
BOOT_SRC_DIR = boot

RUST_TARGET = aarch64-unknown-none
RUST_BUILD_DIR = target/$(RUST_TARGET)/debug

all: kernel8.img

clean:
	rm -rf $(BUILD_DIR)
	cargo clean

$(BUILD_DIR)/boot_s.o: $(BOOT_SRC_DIR)/boot.S
	mkdir -p $(BUILD_DIR)
	$(ARMGNU)-gcc -MMD -c $< -o $@

rust-build:
	cargo build

$(BUILD_DIR)/kernel8.img: $(BOOT_SRC_DIR)/linker.ld $(BUILD_DIR)/boot_s.o rust-build
	$(ARMGNU)-ld \
		-T $(BOOT_SRC_DIR)/linker.ld \
		-o \
			$(BUILD_DIR)/kernel8.elf \
			$(BUILD_DIR)/boot_s.o \
			$(RUST_BUILD_DIR)/librpi_os.a

	$(ARMGNU)-objcopy $(BUILD_DIR)/kernel8.elf -O binary $(BUILD_DIR)/kernel8.img

install-toolchain:
	sudo apt install gcc-aarch64-linux-gnu -y
	rustup target add aarch64-unknown-none

run: $(BUILD_DIR)/kernel8.img
	qemu-system-aarch64 -M raspi3 -kernel $(BUILD_DIR)/kernel8.elf -serial stdio

.PHONY: install-toolchain run rust-build