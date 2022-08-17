use arma_rs::{Group, IntoArma, Value};

#[derive(Default, PartialEq, Eq, Debug)]
pub struct MemoryReport {
    total: u64,
    free: u64,
    avail: u64,
}

impl IntoArma for MemoryReport {
    fn to_arma(&self) -> Value {
        Value::Array(
            vec![self.total, self.free, self.avail]
                .into_iter()
                .map(|v| v.to_string().to_arma())
                .collect(),
        )
    }
}

pub fn memory() -> MemoryReport {
    if let Ok(info) = sys_info::mem_info() {
        MemoryReport {
            total: info.total,
            free: info.free,
            avail: info.avail,
        }
    } else {
        MemoryReport::default()
    }
}

pub fn group() -> Group {
    Group::new().command("memory", memory)
}

#[cfg(test)]
mod tests {
    use arma_rs::Extension;
    #[test]
    fn test_memory() {
        let extension = Extension::build()
            .group("system", super::group())
            .finish()
            .testing();
        let (report, code) = unsafe { extension.call("system:memory", Some(vec![])) };
        assert_eq!(code, 0);
        assert_eq!(report.chars().filter(|c| *c == ',').count(), 2);
    }
}
