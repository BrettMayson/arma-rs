use arma_rs::{rv, rv_handler};

#[macro_use]
extern crate arma_rs;

#[rv]
fn hello() -> &'static str {
    "Hello from Rust!"
}

#[rv]
fn is_arma3(version: u8) -> bool {
    version == 3
}

#[rv(thread = true)]
fn calculate() {
    rv_callback!("test", "myEvent", "test data", 10.5, "more data");
}

#[rv_handler]
fn main() {}
