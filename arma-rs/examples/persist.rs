use std::cell::RefCell;

use arma_rs::{arma, Extension};

#[arma]
fn init() -> Extension<u32> {
    Extension::build_with_persist(0)
        .command("count", count)
        .finish()
}

pub fn count(state: &RefCell<u32>) -> u32 {
    let mut state = state.borrow_mut();
    *state += 1;
    *state
}

#[cfg(test)]
mod tests {
    use super::init;

    #[test]
    fn count() {
        let extension = init().testing();
        let (result, _) = unsafe { extension.call("count", None) };
        assert_eq!(result, "1");
        let (result, _) = unsafe { extension.call("count", None) };
        assert_eq!(result, "2");
    }
}

// Only required for cargo, don't include in your library
fn main() {}
