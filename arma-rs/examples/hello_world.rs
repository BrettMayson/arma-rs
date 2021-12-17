use arma_rs::{arma, Extension};

#[arma]
fn init() -> Extension {
    Extension::new()
        .version("1.0.0".to_string())
        .command("hello", hello)
        .command("welcome", welcome)
        .finish()
}

pub fn hello() -> &'static str {
    "Hello"
}

pub fn welcome(name: String) -> String {
    format!("Welcome {}", name)
}

// Only required for cargo, don't include in your library
fn main() {}
