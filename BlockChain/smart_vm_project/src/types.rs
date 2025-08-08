
pub type Address = [u8; 20];

#[derive(Debug)]
pub enum VMType {
    WASM,
    EVM,
}