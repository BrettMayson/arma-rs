use arma_rs::{Context, Group};

pub fn arma_call(ctx: Context) -> Result<String, String> {
    if let Some(call_ctx) = ctx.arma_call() {
        Ok(format!(
            "{:?},{:?},{:?},{:?}",
            call_ctx.caller(),
            call_ctx.source(),
            call_ctx.mission(),
            call_ctx.server()
        ))
    } else {
        Err("Arma version needs to be 2.11 or higher".to_string())
    }
}

pub fn group() -> Group {
    Group::new().command("arma_call", arma_call)
}

#[cfg(test)]
mod tests {
    use arma_rs::{context, Extension};

    #[test]
    fn test_arma_call_context() {
        let extension = Extension::build()
            .group("context", super::group())
            .finish()
            .testing();
        let (result, code) = unsafe {
            extension.call_with_context(
                "context:arma_call",
                None,
                context::ArmaCallContext::new(
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
