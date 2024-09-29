use std::env;
use std::process::Command;

fn main() {
    let _ = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .spawn();
}
