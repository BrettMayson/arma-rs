use arma_rs::{ArmaValue, Group, IntoArma};

#[derive(Default)]
pub struct MemoryReport {
    total: u64,
    free: u64,
    avail: u64,
}

impl IntoArma for MemoryReport {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Array(
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
    Group::new("system").command("memory", memory)
}
