use std::{thread, time::Duration};

use arma_rs::{Context, Group};

pub fn sleep(ctx: Context, duration: u64, id: String) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(duration));
        ctx.callback_data("timer:sleep", "done", Some(id)).unwrap();
    });
}

pub fn group() -> Group {
    Group::new().command("sleep", sleep)
}

#[cfg(test)]
mod tests {
    use arma_rs::{Extension, Result, Value};
    use std::{thread, time::Duration};

    #[test]
    fn sleep_1sec() {
        let extension = Extension::build()
            .group("timer", super::group())
            .finish()
            .testing();
        let (_, code) = extension.call(
            "timer:sleep",
            Some(vec!["1".to_string(), "test".to_string()]),
        );
        assert_eq!(code, 0);
        let result = extension.callback_handler(
            |name, func, data| {
                assert_eq!(name, "timer:sleep");
                assert_eq!(func, "done");
                if let Some(Value::String(s)) = data {
                    Result::Ok(s)
                } else {
                    Result::Err("Data was not a string".to_string())
                }
            },
            Duration::from_secs(2),
        );
        assert_eq!(Result::Ok("test".to_string()), result);
    }

    #[test]
    fn failed_callback() {
        let extension = Extension::build()
            .group("timer", super::group())
            .finish()
            .testing();
        let (_, code) = extension.call(
            "timer:sleep",
            Some(vec!["3".to_string(), "test".to_string()]), // 3 second sleep causes callback to timeout
        );
        assert_eq!(code, 0);
        let result = extension.callback_handler(
            |name, func, data| {
                assert_eq!(name, "timer:sleep");
                assert_eq!(func, "done");
                if let Some(Value::String(s)) = data {
                    Result::Ok(s)
                } else {
                    Result::Err("Data was not a string".to_string())
                }
            },
            Duration::from_secs(2),
        );
        assert!(result == Result::Timeout);

        // Ignore this for the example, only required so that ci (miri) doesn't complain.
        // Wait for background thread to finish, combined with the 2 second timeout, this should be enough.
        thread::sleep(Duration::from_secs(2));
    }
}
