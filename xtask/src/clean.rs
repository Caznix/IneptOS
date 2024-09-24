use std::process::Command;
use std::fs;

pub fn clean(target: &str) {
    clean_files(target);
    clean_kernel();
}

fn clean_files(target: &str) {
    //ineptos-x86_64

    let image_name = format!("ineptos-{}", {target});
    let iso = &format!("{}.iso", image_name);
    let hdd = &format!("{}.hdd", image_name);

    let _ = fs::remove_dir("iso_root");
    let _ = fs::remove_dir(iso);
    let _ = fs::remove_dir(hdd);
}

fn clean_kernel() {
	//cargo clean
	//rm -rf kernel

    let _ = Command::new("cargo")
        .arg("clean")
        .output()
        .unwrap();

    let _ = fs::remove_dir_all("kernel");
}