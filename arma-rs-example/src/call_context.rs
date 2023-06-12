use arma_rs::{Context, Group};

pub fn current(ctx: Context) -> String {
    format!(
        "{:?},{:?},{:?},{:?}",
        ctx.caller(),
        ctx.source(),
        ctx.mission(),
        ctx.server()
    )
}

pub fn group() -> Group {
    Group::new().command("current", current)
}

#[cfg(test)]
mod tests {
    use arma_rs::{Caller, Extension, Mission, Server, Source};

    #[test]
    fn test_current() {
        let extension = Extension::build()
            .group("context", super::group())
            .finish()
            .testing();
        let (result, code) = unsafe {
            extension.call_with_context(
                "context:current",
                None,
                Caller::Unknown,
                Source::Console,
                Mission::None,
                Server::Singleplayer,
            )
        };
        assert_eq!(code, 0);
        assert_eq!(result, "Unknown,Console,None,Singleplayer");
    }
}
