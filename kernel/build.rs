use std::env;
use std::process::Command;

fn main() {
    let _ = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .spawn();

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:rustc-link-arg=-Tkernel/lds/{arch}-qemu.ld");
    println!("cargo:rerun-if-changed={arch}-qemu.ld");
}
