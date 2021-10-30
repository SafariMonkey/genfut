extern crate cc;

fn main() {
    // Sequential C support
    #[cfg(feature = "sequential_c")]
    cc::Build::new()
        .file("./lib_sequential_c/a.c")
        .flag("-fPIC")
        .flag("-std=c99")
        .shared_flag(true)
        .warnings(false)
        .compile("a");

    // CUDA support
    #[cfg(feature = "cuda")]
    cc::Build::new()
        .file("./lib_cuda/a.c")
        .cuda(true)
        .flag("-Xcompiler")
        .flag("-fPIC")
        .flag("-std=c++03")
        .flag("-w")
        .shared_flag(true)
        .compile("a");
    #[cfg(feature = "cuda")]
    {
        println!("cargo:rustc-link-search=native=##CUDA_INCLUDE_PATH##");
        println!("cargo:rustc-link-search=native=##CUDA_LIBRARY_PATH##");
        println!("cargo:rustc-link-lib=dylib=cuda");
        println!("cargo:rustc-link-lib=dylib=nvrtc");
    }

    // OpenCL support

    #[cfg(feature = "opencl")]
    {
        #[cfg(not(target_os = "macos"))]
        {
            cc::Build::new()
                .file("./lib_opencl/a.c")
                .include("##OPENCL_INCLUDE_PATH##")
                .flag("-fPIC")
                .flag("-std=c99")
                .shared_flag(true)
                .compile("a");
            println!("cargo:rustc-link-lib=dylib=OpenCL");
            println!("cargo:rustc-link-search=native=##OPENCL_LIBRARY_PATH##");
        }
        #[cfg(target_os = "macos")]
        {
            cc::Build::new()
                .file("./lib_opencl/a.c")
                .include("##OPENCL_INCLUDE_PATH##")
                .flag("-fPIC")
                .flag("-std=c99")
                .shared_flag(true)
                .compile("a");
            println!("cargo:rustc-link-lib=framework=OpenCL");
            println!("cargo:rustc-link-search=native=##OPENCL_LIBRARY_PATH##");
        }
    }
}
