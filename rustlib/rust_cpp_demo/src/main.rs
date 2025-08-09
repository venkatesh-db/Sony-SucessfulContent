#[cxx::bridge]
mod ffi {
   unsafe extern "C++" {
        include!("rust_cpp_demo/hello.h"); // relative to include path in build.rs
        fn greet(name: &str) -> String;
    }
}

fn main() {
    let msg = ffi::greet("Venkatesh");
    println!("{}", msg);
}
