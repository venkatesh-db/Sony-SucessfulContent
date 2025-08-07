
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;

/// Our simple Rust kernel module.
module! {
    type: HelloModule,
    name: b"rust_hello_kernel",
    author: b"Venkatesh",
    description: b"Simple Rust Hello World Kernel Module",
    license: b"GPL",
}

struct HelloModule;

impl KernelModule for HelloModule {
    fn init() -> Result<Self> {
        pr_info!("RustHello: module loaded into kernel.\n");
        Ok(HelloModule)
    }
}

impl Drop for HelloModule {
    fn drop(&mut self) {
        pr_info!("RustHello: module unloaded from kernel.\n");
    }
}

/*
make
sudo insmod rust_hello_kernel.ko
dmesg | tail
sudo rmmod rust_hello_kernel
dmesg | tail
*/