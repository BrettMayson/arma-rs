use arma_rs::{arma, Extension};

#[arma]
fn init() -> Extension {
    Extension::build()
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

#[cfg(test)]
mod tests {
    use super::init;

    #[test]
    fn hello() {
        let extension = init().testing();
        let (result, _) = unsafe { extension.call("hello", None) };
        assert_eq!(result, "Hello");
    }

    #[test]
    fn welcome() {
        let extension = init().testing();
        let (result, _) = unsafe { extension.call("welcome", Some(vec!["John".to_string()])) };
        assert_eq!(result, "Welcome John");
    }
}

// Only required for cargo, don't include in your library
fn main() {}
