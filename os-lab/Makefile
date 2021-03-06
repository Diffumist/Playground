TARGET      := riscv64gc-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/os-lab
BIN_FILE    := target/$(TARGET)/$(MODE)/kernel.bin

OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64

.PHONY: doc kernel build clean qemu run

build: $(BIN_FILE)

kernel:
	@cargo build

$(BIN_FILE): kernel
	@$(OBJCOPY) $(KERNEL_FILE) --strip-all -O binary $@

asm:
	@$(OBJDUMP) -d $(KERNEL_FILE) | less

clean:
	@cargo clean

qemu: build
	@qemu-system-riscv64 \
			-machine virt \
			-nographic \
			-bios ./src/bin/rustsbi-qemu.bin \
			-device loader,file=$(BIN_FILE),addr=0x80200000

run: build qemu