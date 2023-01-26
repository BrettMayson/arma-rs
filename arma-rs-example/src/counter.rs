use arma_rs::{Context, Group};

#[derive(Clone)]
struct Counter(usize);

pub fn increment(ctx: Context) {
    let counter = ctx.state().try_get::<Counter>().unwrap_or(Counter(0));
    ctx.state().set(Counter(counter.0 + 1));
}

pub fn group() -> Group {
    Group::new().command("increment", increment)
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

        let (_, code) = unsafe { extension.call("counter:increment", None) };
        let counter = extension.state().get::<super::Counter>();
        assert_eq!(code, 0);
        assert_eq!(counter.0, 1);

        let (_, code) = unsafe { extension.call("counter:increment", None) };
        let counter = extension.state().get::<super::Counter>();
        assert_eq!(code, 0);
        assert_eq!(counter.0, 2);
    }
}
