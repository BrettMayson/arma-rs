use arma_rs::{arma, Extension, Group};

mod system_info;
mod timer;

#[arma]
fn init() -> Extension {
    Extension::build()
        .group(
            "hello",
            Group::new()
                .command("english", hello::english)
                .command("french", hello::french)
                .command("spanish", hello::spanish),
        )
        .group(
            "welcome",
            Group::new()
                .command("english", welcome::english)
                .command("french", welcome::french)
                .command("spanish", welcome::spanish),
        )
        .group("system", system_info::group())
        .group("timer", timer::group())
        .finish()
}

mod hello {
    pub fn english() -> &'static str {
        "hello"
    }

    pub fn french() -> &'static str {
        "bonjour"
    }

    pub fn spanish() -> &'static str {
        "hola"
    }
}

mod welcome {
    pub fn english(name: String) -> String {
        format!("Welcome {}", name)
    }

    pub fn french(name: String) -> String {
        format!("Bienvenue {}", name)
    }

    pub fn spanish(name: String) -> String {
        format!("Bienvenido {}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::init;

    #[test]
    fn hello() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("hello:english", None) };
        assert_eq!(output, "hello");
        let (output, _) = unsafe { extension.call("hello:french", None) };
        assert_eq!(output, "bonjour");
        let (output, _) = unsafe { extension.call("hello:spanish", None) };
        assert_eq!(output, "hola");
    }

    #[test]
    fn welcome() {
        let extension = init().testing();
        let (output, _) =
            unsafe { extension.call("welcome:english", Some(vec!["John".to_string()])) };
        assert_eq!(output, "Welcome John");
        let (output, _) =
            unsafe { extension.call("welcome:french", Some(vec!["John".to_string()])) };
        assert_eq!(output, "Bienvenue John");
        let (output, _) =
            unsafe { extension.call("welcome:spanish", Some(vec!["John".to_string()])) };
        assert_eq!(output, "Bienvenido John");
    }
}
