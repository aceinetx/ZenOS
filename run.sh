set -e
./build.sh
cp ./target/x86_64-unknown-uefi/debug/*.efi ./qemu-testing/esp/efi/boot/bootx64.efi
cd qemu-testing
./run.sh
