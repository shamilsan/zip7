use std::env;
use std::path::Path;

const C_FILES: &[&str] = &["Alloc.c", "CpuArch.c"];

const CPP_FILES: &[&str] = &[
    "7zip/Common/InBuffer.cpp",
    "7zip/Common/OutBuffer.cpp",
    "7zip/Compress/BitlDecoder.cpp",
    "7zip/Compress/ShrinkDecoder.cpp",
    "Common/NewHandler.cpp",
    "Common/MyWindows.cpp",
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.cpp");
    println!("cargo:rerun-if-changed=wrapper.h");

    bindgen();
    build();
}

fn bindgen() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings");

    let out_dir = env::var("OUT_DIR").expect("No `OUT_DIR` env var");
    let out_path = Path::new(&out_dir).join("bindgen.rs");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn build() {
    let c_files = C_FILES.iter().map(|c_file| format!("libzip7/C/{c_file}"));

    let src_files = CPP_FILES
        .iter()
        .map(|cpp_file| format!("libzip7/CPP/{cpp_file}"))
        .chain(c_files);

    let mut build = cc::Build::new();
    build
        .files(src_files)
        .file("wrapper.cpp")
        .cpp(true)
        .define("SHOW_DEBUG_INFO", None)
        .opt_level(3);

    build.compile("libzip7");
}
