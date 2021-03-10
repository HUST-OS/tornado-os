target := "riscv64imac-unknown-none-elf"
mode := "debug"
build-path := "target/" + target + "/" + mode + "/"

bootloader-elf := "../rustsbi/target/" + target + "/debug/rustsbi-qemu"
bootloader-bin := "../rustsbi/target/" + target + "/debug/rustsbi-qemu.bin"
kernel-elf := build-path + "tornado-kernel"
kernel-bin := build-path + "tornado-kernel.bin"

objdump := "riscv64-unknown-elf-objdump"
gdb := "riscv64-unknown-elf-gdb"
size := "rust-size"

threads := "1"

build:
    @just -f "tornado-kernel/justfile" build
    
qemu: build
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios none \
            -device loader,file={{bootloader-bin}},addr=0x80000000 \
            -device loader,file={{kernel-bin}},addr=0x80200000 \
            -smp threads={{threads}}

run: build qemu

asm: build
    @{{objdump}} -D {{kernel-elf}} | less

size: build
    @{{size}} -A -x {{kernel-elf}} 


debug: build
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios none \
            -device loader,file={{bootloader-bin}},addr=0x80000000 \
            -device loader,file={{kernel-bin}},addr=0x80200000 \
            -smp threads={{threads}} \
            -gdb tcp::1234 -S
            
gdb: 
    @{{gdb}} --eval-command="file {{kernel-elf}}" --eval-command="target remote localhost:1234"
