```rust,skt-empty
use arma_rs::{{Context, Extension, IntoArma, Value}};
{}
fn main() {{}}
```

```rust,skt-call-init
use arma_rs::{{loadout::*}};
fn main() {{
    let ext = init();
}}

{}
```

```rust,skt-group
fn main() {{
    let ext = arma_rs::Extension::build().group("test", group()).finish();
}}

{}
```

```rust,skt-main
use arma_rs::{{*, loadout::*}};
fn main() {{
    {}
}}
```

```rust,skt-test
use arma_rs::{{arma, Extension, Group}};

#[arma]
fn init() -> Extension {{
    Extension::build()
        .group(
            "hello",
            Group::new()
                .command("english", hello::english)
        )
        .group(
            "welcome",
            Group::new()
                .command("english", welcome::english)
        )
        .group("timer", timer::group())
        .finish()
}}

fn main() {{
    let ext = init();
}}

mod hello {{
    pub fn english() -> &'static str {{
        "hello"
    }}
}}

mod welcome {{
    pub fn english(name: String) -> String {{
        format!("Welcome {{name}}")
    }}
}}

mod timer {{
    use std::{{thread, time::Duration}};
    use arma_rs::{{Context, Group}};

    pub fn sleep(ctx: Context, duration: u64, id: String) {{
        thread::spawn(move || {{
            thread::sleep(Duration::from_secs(duration));
            ctx.callback_data("timer:sleep", "done", Some(id));
        }});
    }}

    pub fn group() -> Group {{
        Group::new().command("sleep", sleep)
    }}
}}
{}
```
