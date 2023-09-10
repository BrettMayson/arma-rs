use std::sync::atomic::{AtomicU32, Ordering};

use arma_rs::{Context, ContextState, Group};

pub struct Counter(pub AtomicU32);

pub fn increment(ctx: Context) -> Result<(), ()> {
    let Some(counter) = ctx.group().get::<Counter>() else {
        return Err(());
    };
    counter.0.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

pub fn current(ctx: Context) -> Result<u32, ()> {
    let Some(counter) = ctx.group().get::<Counter>() else {
        return Err(());
    };
    Ok(counter.0.load(Ordering::SeqCst))
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

        let (_, code) = extension.call("counter:increment", None);
        assert_eq!(code, 0);
        let (result, code) = extension.call("counter:current", None);
        assert_eq!(code, 0);
        assert_eq!(result, "1");

        let (_, code) = extension.call("counter:increment", None);
        assert_eq!(code, 0);
        let (result, code) = extension.call("counter:current", None);
        assert_eq!(code, 0);
        assert_eq!(result, "2");
    }
}
