target := "riscv64imac-unknown-none-elf"
mode := "debug"
build-path := "target/" + target + "/" + mode + "/"

kernel-elf := build-path + "tornado-kernel"
kernel-bin := build-path + "tornado-kernel.bin"

objdump := "riscv64-unknown-elf-objdump"
size := "rust-size"

threads := "1"

build:
    @just -f "tornado-kernel/justfile" build
    
qemu: build
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios default \
            -device loader,file={{kernel-bin}},addr=0x80200000 \
            -smp threads={{threads}}

run: build qemu

asm: build
    @{{objdump}} -d {{kernel-elf}} | less

size: build
    @{{size}} -A -x {{kernel-elf}} 
