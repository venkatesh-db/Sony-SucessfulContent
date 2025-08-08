
use crate::{wasm_runtime, evm_runtime};

pub struct DistributedRuntime;

impl DistributedRuntime {
    pub fn new() -> Self {
        DistributedRuntime
    }

    pub fn execute_wasm(&mut self, code: &[u8]) {
        println!("\n[Executing WASM Code]");
        wasm_runtime::run(code);
    }

    pub fn execute_evm(&mut self, bytecode: &[u8]) {
        println!("\n[Executing EVM Bytecode]");
        evm_runtime::run(bytecode);
    }
}
