
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::{proc_fs, file, file_operations};
use kernel::net::{self, NetfilterHook, NetfilterHookOps, NetfilterHookReg, NetfilterVerdict};
use kernel::sync::SpinLock;

module! {
    type: RustNetMon,
    name: b"rust_netmon",
    author: b"Venkatesh",
    description: b"Rust Kernel Module for Network Monitoring",
    license: b"GPL",
}

#[derive(Default)]
struct NetStats {
    tcp_count: u64,
    udp_count: u64,
    icmp_count: u64,
    other_count: u64,
}

struct RustNetMon {
    _proc_entry: proc_fs::ProcDirEntry,
    stats: &'static SpinLock<NetStats>,
    hook: Option<NetfilterHookReg>,
}

impl RustNetMon {
    fn new() -> Result<Self> {
        let stats = SpinLock::new(NetStats::default());

        // Create /proc entry
        let proc_entry = proc_fs::ProcFs::new_file(
            cstr!("rust_netmon"),
            0o644,
            None,
            ProcFile,
            stats.clone(),
        )?;

        // Register Netfilter hook
        let hook_ops = NetfilterHookOps {
            hook: packet_hook,
            pf: net::AF_INET,
            priority: net::NF_IP_PRI_FIRST,
        };

        let hook_reg = NetfilterHook::register(&hook_ops)?;

        Ok(Self {
            _proc_entry: proc_entry,
            stats,
            hook: Some(hook_reg),
        })
    }
}

impl KernelModule for RustNetMon {
    fn init() -> Result<Self> {
        pr_info!("RustNetMon: Loading network monitor\n");
        Self::new()
    }
}

impl Drop for RustNetMon {
    fn drop(&mut self) {
        pr_info!("RustNetMon: Unloading module\n");
    }
}

struct ProcFile;

impl file::FileOpener<&'static SpinLock<NetStats>> for ProcFile {
    fn open(ctx: &&'static SpinLock<NetStats>) -> Result<Self::Wrapper> {
        Ok(Box::try_new((Self, *ctx))?)
    }
}

impl file::FileOperations for (ProcFile, &'static SpinLock<NetStats>) {
    kernel::declare_file_operations!(read);

    fn read(
        &self,
        _file: &file::File,
        data: &mut file_operations::UserSlicePtrWriter,
        _offset: u64,
    ) -> Result<usize> {
        let stats = self.1.lock();

        let output = alloc::format!(
            "RustNetMon Stats:\nTCP: {}\nUDP: {}\nICMP: {}\nOther: {}\n",
            stats.tcp_count,
            stats.udp_count,
            stats.icmp_count,
            stats.other_count
        );

        data.write(output.as_bytes())
    }
}

// ------------------- Packet Hook -------------------
fn packet_hook(_priv: &(), skb: &net::SkBuff) -> NetfilterVerdict {
    if let Some(ip_hdr) = skb.ip_header() {
        match ip_hdr.protocol {
            6 => increment_stat(Protocol::TCP),
            17 => increment_stat(Protocol::UDP),
            1 => increment_stat(Protocol::ICMP),
            _ => increment_stat(Protocol::Other),
        }
    }
    NetfilterVerdict::Accept
}

#[derive(Copy, Clone)]
enum Protocol {
    TCP,
    UDP,
    ICMP,
    Other,
}

fn increment_stat(proto: Protocol) {
    static STATS: SpinLock<NetStats> = SpinLock::new(NetStats {
        tcp_count: 0,
        udp_count: 0,
        icmp_count: 0,
        other_count: 0,
    });

    let mut stats = STATS.lock();
    match proto {
        Protocol::TCP => stats.tcp_count += 1,
        Protocol::UDP => stats.udp_count += 1,
        Protocol::ICMP => stats.icmp_count += 1,
        Protocol::Other => stats.other_count += 1,
    }
}

/*

sudo apt update
sudo apt install build-essential linux-headers-$(uname -r) rustc cargo


make
sudo insmod rust_netmon.ko
cat /proc/rust_netmon
ping -c 2 8.8.8.8         # Generates ICMP
curl http://example.com   # Generates TCP
cat /proc/rust_netmon     # See updated counts
sudo rmmod rust_netmon


*/