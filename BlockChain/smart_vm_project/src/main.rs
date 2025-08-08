mod runtime;
mod wasm_runtime;
mod evm_runtime;
mod types;

use runtime::DistributedRuntime;

fn main() {
    let mut runtime = DistributedRuntime::new();
    
    // Example WASM execution
    let wasm_code = b"\0asm..."; // Placeholder for actual WASM binary
    runtime.execute_wasm(wasm_code);

    // Example EVM bytecode execution
    let evm_bytecode = vec![0x60, 0x0A, 0x60, 0x0B, 0x01]; // PUSH1 0x0A, PUSH1 0x0B, ADD
    runtime.execute_evm(&evm_bytecode);
}