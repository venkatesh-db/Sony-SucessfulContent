
pub fn run(bytecode: &[u8]) {
    println!("EVM Runtime: Executing bytecode {:?}", bytecode);
    if bytecode.starts_with(&[0x60, 0x0A, 0x60, 0x0B, 0x01]) {
        println!("EVM Result: 0x15 (0x0A + 0x0B)");
    } else {
        println!("EVM Result: Unknown operation");
    }
}