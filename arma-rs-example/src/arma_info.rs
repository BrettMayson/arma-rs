use arma_rs::{Context, Group};

pub fn arma(ctx: Context) -> Result<String, String> {
    if let Some(arma_info) = ctx.arma_info() {
        Ok(format!(
            "{:?},{:?},{:?},{:?}",
            arma_info.caller(),
            arma_info.source(),
            arma_info.mission(),
            arma_info.server()
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
    fn test_arma_info() {
        let extension = Extension::build()
            .group("info", super::group())
            .finish()
            .testing();
        let (result, code) = unsafe {
            extension.call_with_info(
                "info:arma",
                None,
                context::ArmaInfo::new(
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
