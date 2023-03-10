use arma_rs::{Context, Group};

pub fn arma(ctx: Context) -> Result<String, String> {
    if let Some(arma_ctx) = ctx.arma() {
        Ok(format!(
            "{:?},{:?},{:?},{:?}",
            arma_ctx.caller(),
            arma_ctx.source(),
            arma_ctx.mission(),
            arma_ctx.server()
        ))
    } else {
        Err("Arma version needs to be 2.11 or higher".to_string())
    }
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
                    context::Mission::None,
                    context::Server::Singleplayer,
                ),
            )
        };
        assert_eq!(code, 0);
        assert_eq!(result, "Unknown,Console,None,Singleplayer");
    }
}
