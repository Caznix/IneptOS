use std::process::Command;

fn main() {
    let init = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .spawn();
    let configure = Command::new("cd")
        .arg("limine")
        .arg("./configure")
        .arg("--enable-all")
        .spawn();
}
