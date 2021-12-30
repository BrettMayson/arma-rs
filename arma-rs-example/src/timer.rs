use std::{thread, time::Duration};

use arma_rs::{Context, Group};

pub fn sleep(ctx: Context, duration: u64, id: String) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(duration));
        ctx.callback("timer:sleep", "done", Some(id));
    });
}

pub fn group() -> Group {
    Group::new().command("sleep", sleep)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use arma_rs::{ArmaValue, Extension};

    #[test]
    fn sleep_1sec() {
        let extension = Extension::build()
            .group("timer", super::group())
            .finish()
            .testing();
        let (_, code) = unsafe {
            extension.call(
                "timer:sleep",
                Some(vec!["1".to_string(), "test".to_string()]),
            )
        };
        assert_eq!(code, 0);
        assert!(extension.callback_handler(
            |name, func, data| {
                assert_eq!(name, "timer:sleep");
                assert_eq!(func, "done");
                assert_eq!(data, Some(ArmaValue::String("test".to_string())));
                true
            },
            Duration::from_secs(2)
        ));
    }
}
