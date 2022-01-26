# arma-rs

[![codecov](https://codecov.io/gh/BrettMayson/arma-rs/branch/main/graph/badge.svg?token=A1H7SEZ434)](https://codecov.io/gh/BrettMayson/arma-rs)

The best way to make Arma 3 Extensions.

## Usage

```toml
[dependencies]
arma-rs = "1.3"
```

### Hello World

```rust
use arma_rs::{arma, Extension};

#[arma]
fn init() -> Extension {
    Extension::build()
        .command("hello", hello)
        .command("welcome", welcome)
        .finish()
}

pub fn hello() -> &'static str {
    "Hello"
}

pub fn welcome(name: String) -> String {
    format!("Welcome {}", name)
}
```

```sqf
"my_extension" callExtension ["hello", []]; // Returns ["Hello", 0, 0]
"my_extension" callExtension ["welcome", ["John"]]; // Returns ["Welcome John", 0, 0]
```

## Command Groups

The main reason behind the arma-rs rewrite, command groups! Commands can now be grouped together, making your large projects much easier to manage.

```rust
use arma_rs::{arma, Extension, Group};

mod system_info;
mod timer;

#[arma]
fn init() -> Extension {
    Extension::build()
        .group("hello",
            Group::new()
                .command("english", hello::english)
                .group("english",
                    Group::new()
                        .command("casual", hello::english_casual)
                )
                .command("french", hello::french),
        )
        .group("welcome",
            Group::new()
                .command("english", welcome::english)
                .command("french", welcome::french),
        )
        .finish()
}

mod hello {
    pub fn english() -> &'static str {
        "Hello"
    }
    pub fn english_casual() -> &'static str {
        "Hey"
    }
    pub fn french() -> &'static str {
        "Bonjour"
    }
}

mod welcome {
    pub fn english(name: String) -> String {
        format!("Welcome {}", name)
    }
    pub fn french(name: String) -> String {
        format!("Bienvenue {}", name)
    }
}
```

Commands groups are called by using the format `group:command`. You can nest groups as much as you want.

```sqf
"my_extension" callExtension ["hello:english", []]; // Returns ["Hello", 0, 0]
"my_extension" callExtension ["hello:english:casual", []]; // Returns ["Hey", 0, 0]
"my_extension" callExtension ["hello:french", []]; // Returns ["Bonjour", 0, 0]
```

## Callbacks

Extension callbacks can be invoked anywhere in the extension by adding a variable of type `Context` to the start of a handler.

```rust
use arma_rs::Context;

pub fn sleep(ctx: Context, duration: u64, id: String) {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(duration));
        ctx.callback("example_timer", "done", Some(id));
    });
}

pub fn group() -> arma_rs::Group {
    arma_rs::Group::new().command("sleep", sleep)
}
```

## Custom Return Types

If you're bringing your existing Rust library with your own types, you can easily define how they are converted to Arma.

```rust
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
```

## Error Codes

By default arma-rs will only allow commands via `RvExtensionArgs`. Using `callExtension` with only a function name will return an empty string.

```sqf
"my_extension" callExtension "hello:english" // returns ""
"my_extension" callExtension ["hello:english", []] // returns ["Hello", 0, 0]
```

This behvaiour can be changed by calling `.allow_no_args()` when building the extension. It is recommended not to use this, and to implement error handling instead.

| Code | Description                                       |
|------|---------------------------------------------------|
|  0   | Success                                           |
|  1   | Command not found                                 |
|  2x  | Invalid argument count, x is received count       |
|  3x  | Invalid argument type, x is argument position     |
|  4   | Attempted to write a value larger than the buffer |

### Error Examples

```rust
use arma_rs::{arma, Extension, Context};

#[arma]
fn init() -> Extension {
    Extension::build()
        .command("add", add)
        .finish()
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn overflow(ctx: Context) -> String {
    "X".repeat(ctx.buffer_len() + 1)
}
```

```sqf
"my_extension" callExtension ["add", [1, 2]]; // Returns ["3", 0, 0]
"my_extension" callExtension ["sub", [1, 2]]; // Returns ["", 1, 0]
"my_extension" callExtension ["add", [1, 2, 3]]; // Returns ["", 23, 0], didn't expect 3 elements
"my_extension" callExtension ["add", [1, "two"]]; // Returns ["", 31, 0], unable to parse the second argument
"my_extension" callExtension ["overflow", []]; // Returns 4, the return size was larger than the buffer
```

## Testing

Tests can be created utilizing the `extension.call()` method.

```rust
#[cfg(test)]
mod tests {
    use super::init;

    #[test]
    fn hello() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("hello:english", None) };
        assert_eq!(output, "hello");
    }

    #[test]
    fn welcome() {
        let extension = init().testing();
        let (output, _) =
            unsafe { extension.call("welcome:english", Some(vec!["John".to_string()])) };
        assert_eq!(output, "Welcome John");
    }

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
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
