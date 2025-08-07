#![no_std]
#![no_main]

/*

# Build kernel module (requires Rust-for-Linux toolchain)
make

# Insert module
sudo insmod rust_custom_syscall.ko

# Check device file
ls -l /dev/customsys

*/

use kernel::prelude::*;
use kernel::{chrdev, file_operations, user_ptr::UserSlicePtrWriter, cstr};

module! {
    type: CustomSysModule,
    name: b"rust_custom_syscall",
    author: b"Venkatesh",
    description: b"Rust kernel module simulating a custom syscall via ioctl",
    license: b"GPL",
}

const IOCTL_GET_JIFFIES: u32 = 0x01;

struct CustomSysFile;

impl file_operations::FileOpener<()> for CustomSysFile {
    fn open(_ctx: &(), _file: &kernel::file_operations::File) -> Result<Self::Wrapper> {
        pr_info!("CustomSyscall: device opened\n");
        Ok(CustomSysFile)
    }
}

impl file_operations::FileOperations for CustomSysFile {
    kernel::declare_file_operations!(unlocked_ioctl);

    fn unlocked_ioctl(
        &self,
        _file: &kernel::file_operations::File,
        cmd: u32,
        arg: usize,
    ) -> Result<i32> {
        if cmd == IOCTL_GET_JIFFIES {
            // Get kernel jiffies (ticks since boot)
            let ticks: usize = kernel::time::jiffies() as usize;

            // Write to user-space buffer
            let mut user_writer = unsafe {
                UserSlicePtrWriter::from_raw_parts(arg as *mut u8, core::mem::size_of::<usize>())
            };
            user_writer.write(&ticks.to_ne_bytes())?;

            Ok(0)
        } else {
            Err(Errno::EINVAL)
        }
    }
}

struct CustomSysModule {
    _dev: Pin<Box<chrdev::Registration<()>>>,
}

impl KernelModule for CustomSysModule {
    fn init() -> Result<Self> {
        pr_info!("CustomSyscall: initializing\n");
        let mut chrdev_reg =
            chrdev::Registration::new_pinned::<CustomSysFile>(cstr!("customsys"), 0)?;
        chrdev_reg.as_mut().register()?;
        Ok(CustomSysModule { _dev: chrdev_reg })
    }
}

impl Drop for CustomSysModule {
    fn drop(&mut self) {
        pr_info!("CustomSyscall: module unloaded\n");
    }
}
