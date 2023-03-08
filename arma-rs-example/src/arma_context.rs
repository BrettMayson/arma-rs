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
    use arma_rs::{ArmaContext, Caller, Extension, Mission, Server, Source};

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
                ArmaContext::new(
                    Caller::Unknown,
                    Source::Console,
                    Mission::Unknown,
                    Server::Singleplayer,
                ),
            )
        };
        assert_eq!(code, 0);
        assert_eq!(result, "Unknown,Console,Unknown,Singleplayer");
    }
}
