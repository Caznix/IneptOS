use std::{env::consts::ARCH, process::Command};

#[cfg(target_arch = "x86_64")]
const TARGET_ARCH: &str = "x86_64";
#[cfg(target_arch = "aarch64")]
const TARGET_ARCH: &str = "aarch64";
fn main() {
    // Update submodules
    let init = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .spawn();
    // Configure limine
    let configure = Command::new("cd")
        .arg("limine")
        .arg("./configure")
        .arg("--enable-all")
        .spawn();
    mkiso(TARGET_ARCH);
}
fn mkiso(image_name: &str) {
    std::fs::create_dir("./ovmf");
    //Command::new("curl").args(["-Lo", "$@", ""]).spawn();
}
