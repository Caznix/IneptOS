use std::process::Command;

#[cfg(target_arch = "x86_64")]
const TARGET_ARCH: &str = "x86_64";
#[cfg(target_arch = "aarch64")]
const TARGET_ARCH: &str = "aarch64";
fn main() {
    // Update submodules
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let init = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .spawn();
    // Make the kernel
    //let configure = Command::new("make").arg("all").spawn();
    //mkiso(TARGET_ARCH);
}
fn mkiso(image_name: &str) {
    std::fs::create_dir("./ovmf");
    //Command::new("curl").args(["-Lo", "$@", ""]).spawn();
}
