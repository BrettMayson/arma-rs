use arma_rs::{
    arma_rs_proc::{FromArma, IntoArma},
    Group,
};

#[derive(IntoArma, FromArma)]
pub struct DamagedPart {
    name: String,
    damage: u32,
}

pub fn half_damage(part: DamagedPart) -> DamagedPart {
    DamagedPart {
        damage: part.damage / 2,
        ..part
    }
}

pub fn group() -> Group {
    Group::new().command("half_damage", half_damage)
}

#[cfg(test)]
mod tests {
    use arma_rs::{Extension, FromArma, IntoArma};

    use crate::derive::DamagedPart;

    #[test]
    fn test_half_damage() {
        let extension = Extension::build()
            .group("derive", super::group())
            .finish()
            .testing();

        let damaged_part = super::DamagedPart {
            name: "engine".to_string(),
            damage: 100,
        };
        let (result, code) = extension.call(
            "derive:half_damage",
            Some(vec![damaged_part.to_arma().to_string()]),
        );
        assert_eq!(code, 0);
        assert!(
            result == r#"[["name","engine"],["damage",50]]"#
                || result == r#"[["damage",50],["name","engine"]]"#
        );

        let damaged_part = DamagedPart::from_arma(result).unwrap();
        assert_eq!(damaged_part.name, "engine");
        assert_eq!(damaged_part.damage, 50);
    }
}
