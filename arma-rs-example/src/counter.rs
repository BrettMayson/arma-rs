use std::sync::atomic::{AtomicUsize, Ordering};

use arma_rs::{Context, Group};

pub struct Counter(pub AtomicUsize);

pub fn increment(ctx: Context) {
    let counter = ctx.global().state().get::<Counter>();
    counter.0.fetch_add(1, Ordering::SeqCst);
}

pub fn group() -> Group {
    Group::new().command("increment", increment)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use arma_rs::Extension;

    #[test]
    fn test_counter() {
        let extension = Extension::build()
            .state(super::Counter(0.into()))
            .group("counter", super::group())
            .finish()
            .testing();

        let (_, code) = unsafe { extension.call("counter:increment", None) };
        let counter = extension.state().get::<super::Counter>();
        assert_eq!(code, 0);
        assert_eq!(counter.0.load(Ordering::SeqCst), 1);

        let (_, code) = unsafe { extension.call("counter:increment", None) };
        let counter = extension.state().get::<super::Counter>();
        assert_eq!(code, 0);
        assert_eq!(counter.0.load(Ordering::SeqCst), 2);
    }
}
