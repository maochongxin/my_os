target := riscv32-mcx_os
bbl_path := $(abspath riscv-pk)
mode := debug
kernel := target/$(target)/$(mode)/mcx_os
bin := target/$(target)/$(mode)/kernel.bin

.PHONY: all clean run build asm qemu kernel

all: kernel

$(bin): kernel
	mkdir -p target/$(target)/bbl && \
	cd target/$(target)/bbl && \
	$(bbl_path)/configure \
		--with-arch=rv32imac \
		--disable-fp-emulation \
		--host=riscv64-unknown-elf \
		--with-payload=$(abspath $(kernel)) && \
	make -j32 && \
	cp bbl $(abspath $@)

build: $(bin)

run: build qemu

kernel:
	@cargo xbuild --target riscv32-mcx_os.json
asm:
	@riscv64-unknown-elf-objdump -d $(kernel)
qemu:
	qemu-system-riscv32 -kernel $(bin) -nographic -machine virt

