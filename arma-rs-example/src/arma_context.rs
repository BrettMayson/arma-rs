use arma_rs::{Context, Group};

pub fn arma(ctx: Context) -> String {
    let arma = ctx.arma().unwrap();
    format!(
        "{:?},{:?},{:?},{:?}",
        arma.caller(),
        arma.source(),
        arma.mission(),
        arma.server()
    )
}

pub fn group() -> Group {
    Group::new().command("arma", arma)
}

#[cfg(test)]
mod tests {
    use arma_rs::{context, Extension};

    #[test]
    fn test_arma_context() {
        let extension = Extension::build()
            .group("context", super::group())
            .finish()
            .testing();
        let (result, code) = unsafe {
            extension.call_with_context(
                "context:arma",
                None,
                context::ArmaContext::new(
                    context::Caller::Unknown,
                    context::Source::Console,
                    context::Mission::Unknown,
                    context::Server::Singleplayer,
                ),
            )
        };
        assert_eq!(code, 0);
        assert_eq!(result, "Unknown,Console,Unknown,Singleplayer");
    }
}
