pub fn shim() -> &'static [u8] {
    include_bytes!(env!("CARGO_BIN_FILE_SHIM"))
}

pub fn code() -> &'static [u8] {
    include_bytes!(env!("CARGO_BIN_FILE_CODE"))
}

fn main() {
    let shim = shim();
    let code = code();
    println!("shim {}", env!("CARGO_BIN_FILE_SHIM"));
    println!("code {}", env!("CARGO_BIN_FILE_CODE"));
    println!("shim.len() = {}, code.len() = {}", shim.len(), code.len());
}
