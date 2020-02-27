#[macro_export]
/// Create an `ExtensionCallback` mission event inside Arma 3
/// (name, function, data*)
macro_rules! rv_callback {
    ($n:expr, $f:expr, $($d:expr),*) => {
        use std::any::Any;
        let name = std::ffi::CString::new($n).unwrap().into_raw();
        let func = std::ffi::CString::new($f).unwrap().into_raw();
        let mut out = String::new();
        $(
            let s = $d.to_string();
            let quote = s.parse::<f64>().is_ok();
            if quote {
                out.push('"');
            }
            out.push_str(&s);
            if quote {
                out.push('"');
            }
            out.push_str(",");
        )*
        if out.matches(",").count() == 1 {
            out = out.trim_end_matches(",").to_string();
        } else {
            out = format!("[{}]", out.trim_end_matches(",").trim_matches('"').to_string());
        }
        // let data = if let Some(f) = (&$d as &Any).downcast_ref::<Vec<&str>>() {
        //     println!("`{:?}` is vec.", f);
        //     std::ffi::CString::new(format!("{:?}", $d))
        //         .unwrap()
        //         .into_raw()
        // } else {
        //     println!("I dunno what is `{:?}` :(", $d);
        //     std::ffi::CString::new($d.to_string()).unwrap().into_raw()
        // };
        unsafe {
            rv_send_callback(name, func, std::ffi::CString::new(out).unwrap().into_raw());
        }
    };
}

// commy said no but the dream lives on
//
// #[macro_export]
// /// Fires `CBA_fnc_localEvent`
// /// (name, params)
// ///
// /// params can be one of: Vec<&str>, ToString
// macro_rules! localEvent {
//     ($e:expr, $p:expr) => {
//         use std::any::Any;
//         let params = if let Some(f) = (&$p as &Any).downcast_ref::<Vec<&str>>() {
//             println!("`{:?}` is vec.", f);
//             format!("{:?}", $p)
//         } else {
//             println!("I dunno what is `{:?}` :(", $p);
//             format!("\"{}\"", $p)
//         };
//         unsafe {
//             rv_send_callback(
//                 std::ffi::CString::new("cba_events")
//                     .unwrap()
//                     .into_raw(),
//                 std::ffi::CString::new("localEvent")
//                     .unwrap()
//                     .into_raw(),
//                 std::ffi::CString::new(format!(r#"["{}",{}]"#, $e, params))
//                     .unwrap()
//                     .into_raw(),
//             )
//         }
//     };
// }
