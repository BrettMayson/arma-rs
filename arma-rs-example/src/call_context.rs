use arma_rs::{CallContext, CallContextStackTrace, Group};

pub fn current(call_context: CallContext) -> String {
    format!(
        "{:?},{:?},{:?},{:?},{:?}",
        call_context.caller(),
        call_context.source(),
        call_context.mission(),
        call_context.server(),
        call_context.remote_exec_owner(),
    )
}

pub fn stack(call_context: CallContextStackTrace) -> String {
    format!(
        "{:?},{:?},{:?},{:?},{:?}\n{:?}",
        call_context.caller(),
        call_context.source(),
        call_context.mission(),
        call_context.server(),
        call_context.remote_exec_owner(),
        call_context.stack_trace(),
    )
}

pub fn group() -> Group {
    Group::new()
        .command("current", current)
        .command("stack", stack)
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
        let (result, code) = extension.call_with_context(
            "context:current",
            None,
            Caller::Unknown,
            Source::Console,
            Mission::None,
            Server::Singleplayer,
            0,
        );
        assert_eq!(code, 0);
        assert_eq!(result, "Unknown,Console,None,Singleplayer,0");
    }
}
