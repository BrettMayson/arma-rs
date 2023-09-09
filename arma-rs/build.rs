use std::path::Path;

fn main() {
    let mut root = Path::new("../../README.md");
    if !root.exists() {
        root = Path::new("../README.md");
    }
    if !root.exists() {
        root = Path::new("README.md");
    }
    std::fs::copy(
        root,
        Path::new(&format!("{}/README.md", std::env::var("OUT_DIR").unwrap())),
    )
    .unwrap();
}
