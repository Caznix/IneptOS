use std::process::Command;

use crate::iso::{iso_create_aarch64, iso_create_riscv64, iso_create_x86_64, ovmf_setup};

pub fn qemu_open_x86_64() {
    /*
    qemu-system-x86_64 \
    -M q35 \
    -drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on \
    -drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd \
    -cdrom ineptos-x86_64.iso \
    -m 2G
     */

    ovmf_setup(&"x86_64");
    iso_create_x86_64();

    let _ = Command::new("qemu-system-x86_64")
        .arg("-M")
        .arg("q35")
        .arg("-drive")
        .arg("if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on")
        .arg("-drive")
        .arg("if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd")
        .arg("-cdrom")
        .arg("ineptos-x86_64.iso")
        .arg("-m")
        .arg("2G")
        .status()
        .expect("Failed to execute QEMU");
}

pub fn qemu_open_aarch64() {
    ovmf_setup(&"aarch64");
    iso_create_aarch64();
    unimplemented!();
}

pub fn qemu_open_riscv64() {
    ovmf_setup(&"riscv64");
    iso_create_riscv64();
    unimplemented!();
}
