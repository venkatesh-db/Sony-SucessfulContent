
fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/hello.cpp")
        .flag_if_supported("-std=c++17")
        .include("include")
        .compile("hello_cpp");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/hello.cpp");
    println!("cargo:rerun-if-changed=include/rust_cpp_demo/hello.h");
}
