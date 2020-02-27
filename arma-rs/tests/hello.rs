use arma_rs::{rv, rv_handler};

#[rv]
fn hello() -> &'static str {
    "Hello from Rust!"
}

#[rv]
fn is_arma3(version: u8) -> bool {
    version == 3
}

#[rv_handler]
fn init() {}
