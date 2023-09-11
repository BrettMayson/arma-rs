use arma_rs::{
    arma_rs_proc::{FromArma, IntoArma},
    Group,
};

#[derive(IntoArma, FromArma)]
pub struct Player {
    name: String,
    #[arma(default)]
    is_admin: bool,
}

pub fn dice_roll(player: Player) -> (Player, i8) {
    if player.is_admin {
        (player, 20)
    } else {
        (player, 0)
    }
}

pub fn group() -> Group {
    Group::new().command("dice_roll", dice_roll)
}

#[cfg(test)]
mod tests {
    use arma_rs::Extension;
    #[test]
    fn test_dice_roll() {
        let extension = Extension::build()
            .group("derive", super::group())
            .finish()
            .testing();

        let (result, code) = extension.call(
            "derive:dice_roll",
            Some(vec![r#"[["name","John"]]"#.to_string()]),
        );
        assert_eq!(code, 0);
        assert!(
            result == r#"[[["name","John"],["is_admin",false]],0]"#
                || result == r#"[[["is_admin",false],["name","John"]],0]"#
        );

        let (result, code) = extension.call(
            "derive:dice_roll",
            Some(vec![r#"[["name","John"],["is_admin",true]]"#.to_string()]),
        );
        assert_eq!(code, 0);
        assert!(
            result == r#"[[["name","John"],["is_admin",true]],20]"#
                || result == r#"[[["is_admin",true],["name","John"]],20]"#
        );
    }
}
