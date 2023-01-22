# arma-rs

[Join the arma-rs Discord!](https://discord.gg/qXWUrrwy5d)
[![codecov](https://codecov.io/gh/BrettMayson/arma-rs/branch/main/graph/badge.svg?token=A1H7SEZ434)](https://codecov.io/gh/BrettMayson/arma-rs)

The best way to make Arma 3 Extensions.

## Usage

```toml
[dependencies]
arma-rs = "1.9.1"

[lib]
name = "my_extension"
crate-type = ["cdylib"]
```

### Hello World

```rust
use arma_rs::{arma, Extension};

#[arma]
fn init() -> Extension<()> {
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

Commands can be grouped together, making your large projects much easier to manage.

```rust
use arma_rs::{arma, Extension, Group};

#[arma]
fn init() -> Extension<()> {
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

## Context

Commands can optionally accept a context simply by adding a argument with the type `Context` at the start of the handler.

### State

A state value can be provided when building an extension which then stays persistent between extension invocations, which can then be accessed through `Context`.

```rust
use arma_rs::{arma, Context, Extension};

#[arma]
fn init() -> Extension<u32> {
    Extension::build().state(0).command("count", count).finish()
}

pub fn count(ctx: Context<u32>) -> u32 {
    let mut state = ctx.state().write().unwrap();
    *state += 1;
    *state
}
```

### Callbacks

Extension callbacks can be invoked anywhere in the extension through `Context`.

```rust
use arma_rs::Context;

pub fn sleep(ctx: Context<()>, duration: u64, id: String) {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(duration));
        ctx.callback_data("example_timer", "done", Some(id));
    });
}

pub fn group() -> arma_rs::Group<()> {
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
|  9   | Application error, from using a Result            |

### Error Examples

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn overflow(ctx: Context<()>) -> String {
    "X".repeat(ctx.buffer_len() + 1)
}

pub fn should_error(error: bool) -> Result<String, String> {
  if error {
    Err(String::from("told to error"))
  } else {
    Ok(String::from("told to succeed"))
  }
}
```

```sqf
"my_extension" callExtension ["add", [1, 2]]; // Returns ["3", 0, 0]
"my_extension" callExtension ["sub", [1, 2]]; // Returns ["", 1, 0]
"my_extension" callExtension ["add", [1, 2, 3]]; // Returns ["", 23, 0], didn't expect 3 elements
"my_extension" callExtension ["add", [1, "two"]]; // Returns ["", 31, 0], unable to parse the second argument
"my_extension" callExtension ["overflow", []]; // Returns ["", 4, 0], the return size was larger than the buffer
"my_extension" callExtension ["should_error", [true]]; // Returns ["told to error", 9, 0]
"my_extension" callExtension ["should_error", [false]]; // Returns ["told to succeed", 0, 0]
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

## Unit Loadout Array

arma-rs includes a [loadout module](https://docs.rs/arma-rs/latest/arma_rs/loadout/index.html) to assist with the handling of [Arma's Unit Loadout Array](https://community.bistudio.com/wiki/Unit_Loadout_Array).

```rust
let l = r#"[[],[],[],["U_Marshal",[]],[],[],"H_Cap_headphones","G_Aviator",[],["ItemMap","ItemGPS","","ItemCompass","ItemWatch",""]]"#;
let mut loadout = Loadout::from_arma(l.to_string()).unwrap();
loadout.set_secondary({
    let mut weapon = Weapon::new("launch_B_Titan_short_F".to_string());
    weapon.set_primary_magazine(Magazine::new("Titan_AT".to_string(), 1));
    weapon
});
loadout.set_primary({
    let mut weapon = Weapon::new("arifle_MXC_F".to_string());
    weapon.set_optic("optic_Holosight".to_string());
    weapon
});
let uniform = loadout.uniform_mut();
uniform.set_class("U_B_CombatUniform_mcam".to_string());
let uniform_items = uniform.items_mut().unwrap();
uniform_items.push(InventoryItem::new_item("FirstAidKit".to_string(), 3));
uniform_items.push(InventoryItem::new_magazine("30Rnd_65x39_caseless_mag".to_string(), 5, 30));
```

## Common Rust Libraries

arma-rs supports some common Rust libraries.
You can enable their support by adding their name to the features of arma-rs.

```toml
arma-rs = { version = "1.8.0", features = ["chrono"] }
```

Please create an issue first if you would like to add support for a new library.

### chrono

[`crates.io`](https://crates.io/crates/chrono)

#### chrono - Convert to Arma

[`NaiveDateTime`](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html) and [`DateTime<TimeZone>`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html) will be converted to [Arma's date array](https://community.bistudio.com/wiki/systemTimeUTC).
The timezone will always be converted to UTC.

#### chrono - Convert From Arma

[Arma's date array](https://community.bistudio.com/wiki/systemTimeUTC) can be converted to [`NaiveDateTime`](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html).

### uuid

[`crates.io`](https://crates.io/crates/uuid)

#### uuid - Convert To Arma

[`Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html) will be converted to a string.

### serde_json

[`crates.io`](https://crates.io/crates/serde_json)

#### serde_json - Convert To Arma

Any variant of [`serde_json::Value`](https://docs.serde.rs/serde_json/enum.Value.html) will be converted to the appropriate Arma type.

## Building for x86 (32 Bit)

```sh
rustup toolchain install stable-i686-pc-windows-msvc
cargo +stable-i686-pc-windows-msvc build
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
