@echo off
cargo build --target x86_64-unknown-uefi &&^
xcopy "target/x86_64-unknown-uefi/debug/foam-os.efi" "esp/efi/boot/bootx64.efi" /f /y &&^
qemu-system-x86_64 -device virtio-rng-pci -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd -drive format=raw,file=fat:rw:esp
