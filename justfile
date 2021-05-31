target := "riscv64imac-unknown-none-elf"
mode := "debug"
user-mode := "debug"
build-path := "target/" + target + "/" + mode + "/"
app-path := "target/" + target + "/" + user-mode + "/"

bootloader-bin := "SBI/rustsbi-qemu.bin"
kernel-elf := build-path + "tornado-kernel"
kernel-bin := build-path + "tornado-kernel.bin"
shared-elf := "target/" + target + "/" + mode + "/" + "shared-scheduler"
shared-bin := "target/" + target + "/" + mode + "/" + "shared-scheduler.bin"

objdump := "riscv64-linux-gnu-objdump"
gdb := "riscv64-unknown-elf-gdb"
# gdb := "riscv64-unknown-elf-gdb.exe"
# gdb := "/mnt/d/riscv-binutils-gdb/build_riscv/gdb/gdb"
size := "rust-size"

threads := "1"

build:
    @just -f "tornado-kernel/justfile" build

build-user app:
    @just -f "tornado-user/justfile" build {{app}}

build-shared:
    @just -f "shared-scheduler/justfile" build

qemu app: build build-shared (build-user app)
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios none \
            -device loader,file={{bootloader-bin}},addr=0x80000000 \
            -device loader,file={{kernel-bin}},addr=0x80200000 \
            -device loader,file={{shared-bin}},addr=0x86000000 \
            -device loader,file={{app-path}}{{app}}.bin,addr=0x87000000 \
            -smp threads={{threads}} \

asm: build
    @{{objdump}} -D {{kernel-elf}} | less

asm-shared: build-shared
    @{{objdump}} -D {{shared-elf}} | less

size: build
    @{{size}} -A -x {{kernel-elf}} 
    @{{size}} -A -x {{shared-elf}}

debug app: build build-shared (build-user app)
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios none \
            -device loader,file={{bootloader-bin}},addr=0x80000000 \
            -device loader,file={{kernel-bin}},addr=0x80200000 \
            -device loader,file={{shared-bin}},addr=0x86000000 \
            -device loader,file={{app-path}}{{app}}.bin,addr=0x87000000 \
            -smp threads={{threads}} \
            -gdb tcp::1234 -S \
            
gdb: 
    @{{gdb}} --eval-command="file {{kernel-elf}}" --eval-command="target remote localhost:1234"