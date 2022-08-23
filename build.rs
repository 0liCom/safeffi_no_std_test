extern crate core;

use std::process::Command;

fn main() {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let build_dir = format!("{}{}", project_root, "/cmath");
    println!("Build directory for cmath is: {}", build_dir);

    let build_status = Command::new("./build.sh")
        .current_dir(&build_dir)
        .status()
        .expect("build script of cmath could not be executed")
        .success();

    assert_eq!(build_status, true, "build of cmath failed");

    println!("cargo:rustc-link-search=native=cmath");
    //println!("cargo:rustc-link-search=native=cmatho");
    println!("cargo:rustc-link-lib=static=cmath");
    //println!("cargo:rustc-link-lib=static=cmatho");
}
