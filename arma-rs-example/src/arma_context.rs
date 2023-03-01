use arma_rs::{Context, Group};

pub fn arma(ctx: Context) -> Vec<Option<String>> {
    vec![
        ctx.steam_id().map(String::from),
        ctx.file_source()
            .map(|p| p.to_str().unwrap())
            .map(String::from),
        ctx.mission_name().map(String::from),
        ctx.server_name().map(String::from),
    ]
}

pub fn group() -> Group {
    Group::new().command("arma", arma)
}

#[cfg(test)]
mod tests {
    use arma_rs::{ArmaContext, Extension, IntoArma};

    #[test]
    fn test_arma_context() {
        let mut extension = Extension::build()
            .group("context", super::group())
            .finish()
            .testing();
        let (result, code) = unsafe {
            extension.call_with_context(
                "context:arma",
                None,
                ArmaContext::default()
                    .with_steam_id("steam_id")
                    .with_file_source("file_source")
                    .with_mission_name("mission_name")
                    .with_server_name("server_name"),
            )
        };
        assert_eq!(code, 0);
        assert_eq!(
            result,
            vec!["steam_id", "file_source", "mission_name", "server_name"]
                .to_arma()
                .to_string()
        );
    }
}
