use arma_rs::{rv, rv_callback, rv_handler};

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
    // For use with parseSimpleArray
    rv_callback!("test", "myEvent", "test data", 10.5f32, "more data"); // [""test data"", 10.5, ""more data""]
    rv_callback!("test", "myEvent", "just one data"); // ""just one data""
    rv_callback!("test", "myEvent"); // []

    // Used to send preformatted text that will not be converted to an array
    rv_callback!("test", "myEvent", r#"[""test data"", 10.5, ""more data"""#);
}

#[rv_handler]
fn main() {}
