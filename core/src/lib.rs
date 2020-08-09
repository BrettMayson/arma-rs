pub use arma_rs_codegen::{rv, rv_handler};
pub use libc;

mod to_arma;
pub use to_arma::{ArmaValue, ToArma};

pub fn to_arma<T: ToArma>(t: T) -> ArmaValue {
    t.to_arma()
}

#[macro_export]
/// Create an `ExtensionCallback` mission event inside Arma 3
/// For use with parseSimpleArray when providing multiple arguments
/// (name, function, data*)
macro_rules! rv_callback {
    ($n:expr, $f:expr) => {
        let name = std::ffi::CString::new($n).unwrap().into_raw();
        let func = std::ffi::CString::new($f).unwrap().into_raw();
        unsafe {
            rv_send_callback(name, func, std::ffi::CString::new(String::new()).unwrap().into_raw());
        }
    };
    ($n:expr, $f:expr, $d:expr) => {
        let name = std::ffi::CString::new($n).unwrap().into_raw();
        let func = std::ffi::CString::new($f).unwrap().into_raw();
        let data = std::ffi::CString::new(arma_rs::to_arma($d).to_string().trim_start_matches("\"").trim_end_matches("\"").to_string()).unwrap().into_raw();
        unsafe {
            rv_send_callback(name, func, data);
        }
    };
    ($n:expr, $f:expr, $($d:expr),+) => {
        let name = std::ffi::CString::new($n).unwrap().into_raw();
        let func = std::ffi::CString::new($f).unwrap().into_raw();
        unsafe {
            rv_send_callback(name, func, std::ffi::CString::new(arma_rs::quote!(arma_rs::simple_array!($($d),*))).unwrap().into_raw());
        }
    };
}

#[macro_export]
macro_rules! simple_array {
    ($($d:expr),*) => {{
        let mut v = Vec::new();
        $(
            v.push(arma_rs::to_arma($d));
        )*
        arma_rs::ArmaValue::Array(v)
    }};
}

#[macro_export]
macro_rules! quote {
    ($d:expr) => {
        $d.to_string()
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
//         let params = if let Some(f) = (&$p as &std::any::Any).downcast_ref::<Vec<&str>>() {
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
