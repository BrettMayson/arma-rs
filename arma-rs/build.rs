fn main() {
    if std::env::var("SKEPTIC_SKIP").unwrap_or("0".to_string()) == "1" {
        std::fs::File::create(
            std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("skeptic-tests.rs"),
        )
        .unwrap();
        return;
    }
    let path = std::path::PathBuf::from("../README.md");
    if path.exists() {
        skeptic::generate_doc_tests(&[path]);
    }
}
