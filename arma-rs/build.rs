fn main() {
    let path = std::path::PathBuf::from("../README.md");
    if path.exists() {
        skeptic::generate_doc_tests(&[path]);
    }
}
