use std::fs::create_dir_all;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

pub(crate) fn gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {
    #[cfg(feature = "sequential_c")]
    {
        let out_path = PathBuf::from(out_dir);
        let lib_dir = out_path.join("lib_sequential_c");
        if let Err(e) = create_dir_all(lib_dir.clone()) {
            eprintln!("Error creating {} ({})", lib_dir.display(), e);
            std::process::exit(1);
        }
        let output = Command::new("futhark")
            .arg("c")
            .arg("--library")
            .arg("-o")
            .arg(format!(
                "{}/lib_sequential_c/a",
                out_dir.to_str().expect("[gen_c] out_dir failed!")
            ))
            .arg(in_file)
            .output()
            .expect("[gen_c] failed to execute process");
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    #[cfg(feature = "cuda")]
    {
        let out_path = PathBuf::from(out_dir);
        let lib_dir = out_path.join("lib_cuda");
        if let Err(e) = create_dir_all(lib_dir.clone()) {
            eprintln!("Error creating {} ({})", lib_dir.display(), e);
            std::process::exit(1);
        }
        let output = Command::new("futhark")
            .arg("cuda")
            .arg("--library")
            .arg("-o")
            .arg(format!(
                "{}/lib_cuda/a",
                out_dir.to_str().expect("[gen_c] out_dir failed!")
            ))
            .arg(in_file)
            .output()
            .expect("failed to execute process");
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    #[cfg(feature = "opencl")]
    {
        let out_path = PathBuf::from(out_dir);
        let lib_dir = out_path.join("lib_opencl");
        if let Err(e) = create_dir_all(lib_dir.clone()) {
            eprintln!("Error creating {} ({})", lib_dir.display(), e);
            std::process::exit(1);
        }
        let output = Command::new("futhark")
            .arg("opencl")
            .arg("--library")
            .arg("-o")
            .arg(format!(
                "{}/lib_opencl/a",
                out_dir.to_str().expect("[gen_c] out_dir failed!")
            ))
            .arg(in_file)
            .output()
            .expect("failed to execute process");
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
pub(crate) fn generate_bindings(
    header: &std::path::Path,
    include_path: Option<&str>,
    out: &std::path::Path,
) {
    let bindings = bindgen::Builder::default()
        .header(
            header
                .to_str()
                .expect("[generate_bindings] Error with header!"),
        )
        .clang_args(include_path.map(|path| format!("-I{}", path)))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(out);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
