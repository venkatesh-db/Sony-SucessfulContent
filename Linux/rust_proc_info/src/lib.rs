
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::file_operations;
use kernel::file::{File, FileOpener, FileOperations};
use kernel::proc_fs;

module! {
    type: RustProcModule,
    name: b"rust_proc_info",
    author: b"Venkatesh",
    description: b"Advanced Rust Kernel Module with /proc interface",
    license: b"GPL",
}

struct RustProcModule {
    _proc_entry: proc_fs::ProcDirEntry,
}

struct ProcFile;

impl FileOpener<()> for ProcFile {
    fn open(_ctx: &()) -> Result<Self::Wrapper> {
        Ok(Box::try_new(Self)?)
    }
}

impl FileOperations for ProcFile {
    kernel::declare_file_operations!(read);

    fn read(
        &self,
        _file: &File,
        data: &mut file_operations::UserSlicePtrWriter,
        _offset: u64,
    ) -> Result<usize> {
        // Get uptime from kernel
        let uptime_jiffies = kernel::time::jiffies();
        let uptime_secs = uptime_jiffies / kernel::time::HZ as u64;

        // Get process count
        let task_count = kernel::task::task_count();

        let output = alloc::format!(
            "Rust Kernel Module Info:\nUptime: {} seconds\nRunning Tasks: {}\n",
            uptime_secs,
            task_count
        );

        data.write(output.as_bytes())
    }
}

impl KernelModule for RustProcModule {
    fn init() -> Result<Self> {
        pr_info!("RustProcInfo: loading module\n");

        let proc_entry = proc_fs::ProcFs::new_file(
            cstr!("rust_info"),
            0o644,
            None,
            ProcFile,
            (),
        )?;

        Ok(Self {
            _proc_entry: proc_entry,
        })
    }
}

impl Drop for RustProcModule {
    fn drop(&mut self) {
        pr_info!("RustProcInfo: unloading module\n");
    }
}

/*

make
sudo insmod rust_proc_info.ko
cat /proc/rust_info
sudo rmmod rust_proc_info


*/