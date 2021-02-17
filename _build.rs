//! Build addition that allows us to compile the nvidia kernels into ptx files that can be
//! used for FFI calls later. This is currently not in use.
use std::process::Command;

fn main() {
    // let _out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = "./kernels/compiled";
    let input_dir = "./kernels";
    let source_files = vec!["add", "hello-world"];

    for file in source_files {
        let status = Command::new("nvcc")
            .arg(&format!("{}/{}.cu", input_dir, file))
            .args(&["--ptx", "-o"])
            .arg(&format!("{}/{}.ptx", out_dir, file))
            .status()
            .unwrap();

        if !status.success() {
            println!("nvcc compile of '{}' exited with {}", file, status);
            panic!("nvcc compile failed");
        }
        println!("cargo:rerun-if-changed=kernels/{}.cu", file);
    }
}
