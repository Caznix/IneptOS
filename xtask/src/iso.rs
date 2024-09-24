use std::process::Command;
use std::fs;

use crate::build::{build_aarch64, build_limine, build_riscv64, build_x86_64};

pub fn iso_create_x86_64() {
    build_limine();
    build_x86_64();

    iso_root_setup();
    create_iso_with_bootloader_x86_64();
    iso_root_remove();
}

pub fn iso_create_aarch64() {
    build_limine();
    build_aarch64();

    iso_root_setup();
    create_iso_with_bootloader_aarch64();
    iso_root_remove();
}

pub fn iso_create_riscv64() {
    build_limine();
    build_riscv64();

    iso_root_setup();
    create_iso_with_bootloader_riscv64();
    iso_root_remove();
}

fn iso_root_setup() {
    if fs::metadata("iso_root").is_ok() {
        Command::new("rm")
            .arg("-rf")
            .arg("iso_root")
            .output()
            .expect("Failed to remove iso_root directory");
    }

    fs::create_dir_all("iso_root/boot").unwrap();
    fs::copy("kernel/kernel", "iso_root/boot/kernel").unwrap();
    fs::create_dir_all("iso_root/boot/limine").unwrap();
    fs::copy("limine.conf", "iso_root/boot/limine/limine.conf").unwrap();
    fs::create_dir_all("iso_root/EFI/BOOT").unwrap();
}

fn iso_root_remove() {
    if fs::metadata("iso_root").is_ok() {
        fs::remove_dir_all("iso_root").unwrap();
    }
}

fn create_iso_with_bootloader_x86_64() {
    fs::copy(
        "limine/limine-bios.sys",
        "iso_root/boot/limine/limine-bios-cd.bin",
    )
    .unwrap();

    fs::copy(
        "limine/limine-uefi-cd.bin",
        "iso_root/boot/limine/limine-uefi-cd.bin",
    )
    .unwrap();

    fs::copy("limine/BOOTX64.EFI", "iso_root/EFI/BOOT/BOOTX64.EFI").unwrap();
    fs::copy("limine/BOOTIA32.EFI", "iso_root/EFI/BOOT/BOOTIA32.EFI").unwrap();

    let status = Command::new("xorriso")
        .args(&[
            "-as",
            "mkisofs",
            "-b",
            "boot/limine/limine-bios-cd.bin",
            "-no-emul-boot",
            "-boot-load-size",
            "4",
            "-boot-info-table",
            "--efi-boot",
            "boot/limine/limine-uefi-cd.bin",
            "-efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
            "iso_root",
            "-o",
            &format!("{}.iso", "ineptos-x86_64"),
        ])
        .status()
        .unwrap();

    if !status.success() {
        panic!("xorriso failed to create ISO image");
    }

    let _ = Command::new("./limine/limine")
        .args(&["bios-install", &format!("{}.iso", "ineptos-x86_64")])
        .status()
        .unwrap();
    iso_root_remove();
}

fn create_iso_with_bootloader_aarch64() {
    unimplemented!();
}

fn create_iso_with_bootloader_riscv64() {
    unimplemented!();
}

pub fn ovmf_setup(target: &str) {
    //mkdir -p ovmf
    fs::create_dir_all("ovmf").unwrap();
    ovmf_code_download(target);
    ovmf_vars_download(target);
}

fn ovmf_code_download(target: &str) {
    //curl -Lo ovmf/ovmf-vars-x86_64.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-code-x86_64.fd
    let ovmf_file = format!("ovmf-code-{}.fd", target);
    let ovmf_file_path = format!("ovmf/{}", ovmf_file);

    let url = format!(
        "https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/{}",
        ovmf_file
    );

    let _ = Command::new("curl")
        .arg("-Lo")
        .arg(ovmf_file_path)
        .arg(url)
        .output();
}

fn ovmf_vars_download(target: &str) {
    //curl -Lo ovmf/ovmf-code-x86_64.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-code-x86_64.fd
    let ovmf_file = format!("ovmf-vars-{}.fd", target);
    let ovmf_file_path = format!("ovmf/{}", ovmf_file);
    let url = format!(
        "https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/{}",
        ovmf_file
    );

    let _ = Command::new("curl")
        .arg("-Lo")
        .arg(ovmf_file_path)
        .arg(url)
        .output();
}
