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
    use std::{time::Duration};

    use arma_rs::{Extension, Value};

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
        let (success, result) = extension.callback_handler(
            |name, func, data| {
                assert_eq!(name, "timer:sleep");
                assert_eq!(func, "done");
                assert_eq!(data, Some(Value::String("test".to_string())));
                (true, data.unwrap().to_string())
            },
            Duration::from_secs(2)
        );
        assert!(success);
        assert!(result == "test");
    }

    #[test]
    fn sleep_timeout() {
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
        let (success, result) = extension.callback_handler(
            |name, func, data| {
                assert_eq!(name, "timer:sleep");
                assert_eq!(func, "done");
                assert_eq!(data, Some(Value::String("test".to_string())));
                let result = data.unwrap().as_str().unwrap().to_string();
                (true, result)
            },
            Duration::from_secs(2)
        ); 
        assert!(success);
        assert!(result == "test");
    }
    #[test]
    fn failed_callback() {
        let extension = Extension::build()
            .group("timer", super::group())
            .finish()
            .testing();
        let (_, code) = unsafe {
            extension.call(
                "timer:sleep",
                Some(vec!["600".to_string(), "test".to_string()]), // 10 minute sleep causes callback to timeout
            )
        };
        assert_eq!(code, 0);
        let (success, result) = extension.callback_handler(
            |name, func, data| {
                assert_eq!(name, "timer:sleep");
                assert_eq!(func, "done");
                assert_eq!(data, Some(Value::String("test".to_string())));
                let result = data.unwrap().as_str().unwrap().to_string();
                (true, result)
            },
            Duration::from_secs(2)
        ); 
        assert!(!success);
        assert!(result == String::default());
    }
}
