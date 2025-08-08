use wasmtime::*;
use anyhow::Result;

// --- WASM Runtime ---
fn run_wasm_runtime() -> Result<()> {
    println!("\n[Executing WASM Code]");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let wat = r#"
        (module
            (func $add (param $lhs i32) (param $rhs i32) (result i32)
                local.get $lhs
                local.get $rhs
                i32.add)
            (export "add" (func $add)))
    "#;

    let module = Module::new(&engine, wat)?;
    let instance = Instance::new(&mut store, &module, &[])?;
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let result = add.call(&mut store, (10, 20))?;
    println!("WASM Result (10 + 20): {}", result);

    Ok(())
}

// --- EVM Bytecode Simulation ---
fn run_evm_runtime() {
    println!("\n[Executing EVM Bytecode]");
    // PUSH1 0x0A, PUSH1 0x0B, ADD
    let bytecode = vec![0x60, 0x0A, 0x60, 0x0B, 0x01];
    let mut stack: Vec<u8> = Vec::new();
    let mut pc = 0;

    while pc < bytecode.len() {
        let opcode = bytecode[pc];
        pc += 1;

        match opcode {
            0x60 => {
                // PUSH1
                let val = bytecode[pc];
                pc += 1;
                stack.push(val);
            }
            0x01 => {
                // ADD
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a.wrapping_add(b));
            }
            _ => println!("Unknown opcode: 0x{:x}", opcode),
        }
    }

    if let Some(result) = stack.pop() {
        println!("EVM Result: 0x{:X} (0x0A + 0x0B)", result);
    }
}

fn main() -> Result<()> {
    println!("Running Multi-Runtime Smart Contract System");
    run_wasm_runtime()?;
    run_evm_runtime();
    Ok(())
}
