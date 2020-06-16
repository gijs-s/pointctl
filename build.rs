use std::env;
use std::process::Command;

fn main() {
    let _out_dir = env::var("OUT_DIR").unwrap();

    // Note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    let status = Command::new("nvcc")
        .args(&[
            "./kernels/add.cu",
            "--ptx",
            "-o",
            "./kernels/compiled/add.ptx",
        ])
        .status().unwrap();

    if !status.success() {
        println!("nvcc compile of '{}' exited with {}", "add", status);
        panic!("nvcc compile failed");
    }
    println!("cargo:rerun-if-changed=kernels/add.cu");
}
