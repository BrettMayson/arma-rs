use std::sync::atomic::{AtomicU32, Ordering};

use arma_rs::{Context, Group};

pub struct Counter(pub AtomicU32);

pub fn increment(ctx: Context) {
    let counter = ctx.state().get::<Counter>();
    counter.0.fetch_add(1, Ordering::SeqCst);
}

pub fn current(ctx: Context) -> u32 {
    let counter = ctx.state().get::<Counter>();
    counter.0.load(Ordering::SeqCst)
}

pub fn group() -> Group {
    Group::new()
        .command("increment", increment)
        .command("current", current)
        .state(Counter(0.into()))
}

#[cfg(test)]
mod tests {
    use arma_rs::Extension;

    #[test]
    fn test_counter() {
        let extension = Extension::build()
            .group("counter", super::group())
            .finish()
            .testing();

        unsafe { extension.call("counter:increment", None) };
        let (result, code) = unsafe { extension.call("counter:current", None) };
        assert_eq!(code, 0);
        assert_eq!(result, "1");

        unsafe { extension.call("counter:increment", None) };
        let (result, code) = unsafe { extension.call("counter:current", None) };
        assert_eq!(code, 0);
        assert_eq!(result, "2");
    }
}
