use std::env;
use std::path::Path;

const C_FILES: &[&str] = &[
    "7zCrc.c",
    "7zCrcOpt.c",
    "Aes.c",
    "AesOpt.c",
    "Alloc.c",
    "CpuArch.c",
    "Delta.c",
    "LzmaDec.c",
    "Ppmd8.c",
    "Sha1.c",
    "Sha256.c",
    "Threads.c",
    "Xxh64.c",
    "Xz.c",
    "XzCrc64.c",
    "XzCrc64Opt.c",
    "ZstdDec.c",
];

const CPP_FILES: &[&str] = &[
    "7zip/Archive/Zip/ZipHandler.cpp",
    "7zip/Archive/Zip/ZipIn.cpp",
    "7zip/Archive/Zip/ZipItem.cpp",
    "7zip/Archive/Common/HandlerOut.cpp",
    "7zip/Archive/Common/ItemNameUtils.cpp",
    "7zip/Archive/Common/OutStreamWithCRC.cpp",
    "7zip/Common/CreateCoder.cpp",
    "7zip/Common/CWrappers.cpp",
    "7zip/Common/FileStreams.cpp",
    "7zip/Common/FilterCoder.cpp",
    "7zip/Common/InBuffer.cpp",
    "7zip/Common/LimitedStreams.cpp",
    "7zip/Common/MethodId.cpp",
    "7zip/Common/MethodProps.cpp",
    "7zip/Common/OutBuffer.cpp",
    "7zip/Common/ProgressUtils.cpp",
    "7zip/Common/StreamObjects.cpp",
    "7zip/Common/StreamUtils.cpp",
    "7zip/Common/UniqBlocks.cpp",
    "7zip/Compress/BitlDecoder.cpp",
    "7zip/Compress/CopyCoder.cpp",
    "7zip/Compress/ImplodeDecoder.cpp",
    "7zip/Compress/LzfseDecoder.cpp",
    "7zip/Compress/LzmaDecoder.cpp",
    "7zip/Compress/LzOutWindow.cpp",
    "7zip/Compress/PpmdZip.cpp",
    "7zip/Compress/ShrinkDecoder.cpp",
    "7zip/Compress/XzDecoder.cpp",
    "7zip/Compress/ZstdDecoder.cpp",
    "7zip/Crypto/HmacSha1.cpp",
    "7zip/Crypto/MyAes.cpp",
    "7zip/Crypto/Pbkdf2HmacSha1.cpp",
    "7zip/Crypto/RandGen.cpp",
    "7zip/Crypto/WzAes.cpp",
    "7zip/Crypto/ZipCrypto.cpp",
    "7zip/Crypto/ZipStrong.cpp",
    "Common/C_FileIO.cpp",
    "Common/IntToString.cpp",
    "Common/NewHandler.cpp",
    "Common/MyVector.cpp",
    "Common/MyWindows.cpp",
    "Common/StringConvert.cpp",
    "Common/StringToInt.cpp",
    "Common/UTFConvert.cpp",
    "Windows/FileDir.cpp",
    "Windows/FileFind.cpp",
    "Windows/FileName.cpp",
    "Windows/FileIO.cpp",
    "Windows/PropVariant.cpp",
    "Windows/PropVariantUtils.cpp",
    "Windows/Synchronization.cpp",
    "Windows/System.cpp",
    "Windows/TimeUtils.cpp",
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
        .allowlist_type("Handle")
        .allowlist_function("close_archive")
        .allowlist_function("init")
        .allowlist_function("items_count")
        .allowlist_function("open_archive")
        .allowlist_item("NArchive::NExtract::NOperationResult::kOK")
        .allowlist_item("NArchive::NExtract::NOperationResult::kUnsupportedMethod")
        .allowlist_item("NArchive::NExtract::NOperationResult::kDataError")
        .allowlist_item("NArchive::NExtract::NOperationResult::kCRCError")
        .allowlist_item("NArchive::NExtract::NOperationResult::kUnavailable")
        .allowlist_item("NArchive::NExtract::NOperationResult::kUnexpectedEnd")
        .allowlist_item("NArchive::NExtract::NOperationResult::kDataAfterEnd")
        .allowlist_item("NArchive::NExtract::NOperationResult::kIsNotArc")
        .allowlist_item("NArchive::NExtract::NOperationResult::kHeadersError")
        .allowlist_item("NArchive::NExtract::NOperationResult::kWrongPassword")
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
