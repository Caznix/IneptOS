use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

pub fn build_x86_64() {
    //RUSTFLAGS="-C relocation-model=static"
    //cargo build -j12 --manifest-path kernel/Cargo.toml --target=x86_64-unknown-none
    //cp kernel/target/x86_64-unknown-none/debug/./ineptOS kernel/kernel

    env::set_var("RUSTFLAGS", "-C relocation-model=static");

    let _ = Command::new("cargo")
        .arg("build")
        .arg("-j12")
        .arg("--manifest-path")
        .arg("kernel/Cargo.toml")
        .arg("--target=x86_64-unknown-none")
        .output();

    let target_dir = "target/x86_64-unknown-none/debug";

    let find_output = Command::new("find")
        .arg(&target_dir)
        .arg("-maxdepth")
        .arg("1")
        .arg("-perm")
        .arg("-111")
        .arg("-type")
        .arg("f")
        .output();

    let converted_stdout_result = String::from_utf8(find_output.unwrap().stdout);
    let ineptos_binary_path = PathBuf::from(&converted_stdout_result.unwrap().trim());

    fs::copy(&ineptos_binary_path, &PathBuf::from("kernel/kernel")).unwrap();
}

pub fn build_aarch64() {
    unimplemented!()
}

pub fn build_riscv64() {
    unimplemented!()
}

pub fn build_limine() {
    let _ = Command::new("make").arg("-C").arg("limine").output();
}
