use crate::ext_result::IntoExtResult;
use crate::value::{FromArma, Value};
use crate::Context;

type HandlerFunc = Box<
    dyn Fn(
        Context,
        *mut libc::c_char,
        libc::size_t,
        Option<*mut *mut i8>,
        Option<libc::c_int>,
    ) -> libc::c_int,
>;

/// A wrapper for `HandlerFunc`
pub struct Handler {
    /// The function to call
    pub handler: HandlerFunc,
}

/// Create a new handler from a Factory
pub fn fn_handler<C, I, R>(command: C) -> Handler
where
    C: Factory<I, R> + 'static,
{
    Handler {
        handler: Box::new(
            move |context: Context,
                  output: *mut libc::c_char,
                  size: libc::size_t,
                  args: Option<*mut *mut i8>,
                  count: Option<libc::c_int>|
                  -> libc::c_int {
                unsafe { command.call(context, output, size, args, count) }
            },
        ),
    }
}

/// Execute a command
pub trait Executor: 'static {
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    unsafe fn call(
        &self,
        context: Context,
        output: *mut libc::c_char,
        size: libc::size_t,
        args: Option<*mut *mut i8>,
        count: Option<libc::c_int>,
    );
}

/// A factory for creating a command handler.
/// Creates a handler from any function that optionally takes a context and up to 12 arguments.
/// The arguments must implement `FromArma`
/// The return value must implement `IntoExtResult`
pub trait Factory<A, R> {
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    unsafe fn call(
        &self,
        context: Context,
        output: *mut libc::c_char,
        size: libc::size_t,
        args: Option<*mut *mut i8>,
        count: Option<libc::c_int>,
    ) -> libc::c_int;
}

macro_rules! factory_tuple ({ $c: expr, $($param:ident)* } => {
    impl<$($param,)* O> Executor for dyn Factory<($($param,)*), O>
    where
        O: 'static,
        $($param: FromArma + 'static,)*
    {
        unsafe fn call(
            &self,
            context: Context,
            output: *mut libc::c_char,
            size: libc::size_t,
            args: Option<*mut *mut i8>,
            count: Option<libc::c_int>,
        ) {
            self.call(context, output, size, args, count);
        }
    }

    // No context without return
    impl<Func, $($param,)*> Factory<($($param,)*), ()> for Func
    where
        Func: Fn($($param),*),
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, _: Context, _output: *mut libc::c_char, _size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int{
            let count = count.unwrap_or_else(|| 0);
            if count != $c {
                println!("Invalid number of arguments: expected {}, got {}", $c, count);
                return format!("2{}", count).parse::<libc::c_int>().unwrap();
            }
            if $c == 0 {
                (self)($($param::from_arma("".to_string()).unwrap(),)*);
                0
            } else {
                #[allow(unused_variables, unused_mut)]
                let mut argv: Vec<String> = {
                    let argv: &[*mut libc::c_char; $c] = &*(args.unwrap() as *const [*mut i8; $c]);
                    let mut argv = argv
                    .to_vec()
                    .into_iter()
                    .map(|s|
                        std::ffi::CStr::from_ptr(s)
                        .to_string_lossy()
                        .trim_matches('\"')
                        .to_owned()
                    )
                    .collect::<Vec<String>>();
                    argv.reverse();
                    argv
                };
                #[allow(unused_variables, unused_mut)] // Caused by the 0 loop
                let mut c = 0;
                #[allow(unused_assignments, clippy::eval_order_dependence)]
                (self)($(
                    if let Ok(val) = $param::from_arma(argv.pop().unwrap()) {
                        c += 1;
                        val
                    } else {
                        return format!("3{}", c).parse::<libc::c_int>().unwrap()
                    },
                )*);
                0
            }
        }
    }

    // Context without return
    impl<Func, $($param,)*> Factory<(Context, $($param,)*), ()> for Func
    where
        Func: Fn(Context, $($param),*),
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, context: Context, _output: *mut libc::c_char, _size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int{
            let count = count.unwrap_or_else(|| 0);
            if count != $c {
                println!("Invalid number of arguments: expected {}, got {}", $c, count);
                return format!("2{}", count).parse::<libc::c_int>().unwrap();
            }
            if $c == 0 {
                (self)(context, $($param::from_arma("".to_string()).unwrap(),)*);
                0
            } else {
                #[allow(unused_variables, unused_mut)]
                let mut argv: Vec<String> = {
                    let argv: &[*mut libc::c_char; $c] = &*(args.unwrap() as *const [*mut i8; $c]);
                    let mut argv = argv
                    .to_vec()
                    .into_iter()
                    .map(|s|
                        std::ffi::CStr::from_ptr(s)
                        .to_string_lossy()
                        .trim_matches('\"')
                        .to_owned()
                    )
                    .collect::<Vec<String>>();
                    argv.reverse();
                    argv
                };
                #[allow(unused_variables, unused_mut)] // Caused by the 0 loop
                let mut c = 0;
                #[allow(unused_assignments, clippy::eval_order_dependence)]
                (self)(context,
                $(
                    if let Ok(val) = $param::from_arma(argv.pop().unwrap()) {
                        c += 1;
                        val
                    } else {
                        return format!("3{}", c).parse::<libc::c_int>().unwrap()
                    },
                )*);
                0
            }
        }
    }

    // No context with input and return
    impl<Func, $($param,)* R> Factory<($($param,)*), R> for Func
    where
        R: IntoExtResult + 'static,
        Func: Fn($($param),*) -> R,
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, _: Context, output: *mut libc::c_char, size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int {
            let count = count.unwrap_or_else(|| 0);
            if count != $c {
                println!("Invalid number of arguments: expected {}, got {}", $c, count);
                return format!("2{}", count).parse::<libc::c_int>().unwrap();
            }
            if $c == 0 {
                handle_output_and_return(
                    (self)($($param::from_arma("".to_string()).unwrap(),)*),
                    output,
                    size
                )
            } else {
                #[allow(unused_variables, unused_mut)]
                let mut argv: Vec<String> = {
                    let argv: &[*mut libc::c_char; $c] = &*(args.unwrap() as *const [*mut i8; $c]);
                    let mut argv = argv
                    .to_vec()
                    .into_iter()
                    .map(|s|
                        std::ffi::CStr::from_ptr(s)
                        .to_string_lossy()
                        .trim_matches('\"')
                        .to_owned()
                    )
                    .collect::<Vec<String>>();
                    argv.reverse();
                    argv
                };
                #[allow(unused_variables, unused_mut)] // Caused by the 0 loop
                let mut c = 0;
                #[allow(unused_assignments, clippy::eval_order_dependence)]
                handle_output_and_return(
                    {
                        (self)($(
                            if let Ok(val) = $param::from_arma(argv.pop().unwrap()) {
                                c += 1;
                                val
                            } else {
                                return format!("3{}", c).parse::<libc::c_int>().unwrap()
                            },
                        )*)
                    },
                    output,
                    size
                )
            }
        }
    }

    // Context with input and return
    impl<Func, $($param,)* R> Factory<(Context, $($param,)*), R> for Func
    where
        R: IntoExtResult + 'static,
        Func: Fn(Context, $($param),*) -> R,
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, context: Context, output: *mut libc::c_char, size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int {
            let count = count.unwrap_or_else(|| 0);
            if count != $c {
                println!("Invalid number of arguments: expected {}, got {}", $c, count);
                return format!("2{}", count).parse::<libc::c_int>().unwrap();
            }
            if $c == 0 {
                handle_output_and_return(
                    (self)(context, $($param::from_arma("".to_string()).unwrap(),)*),
                    output,
                    size
                )
            } else {
                #[allow(unused_variables, unused_mut)]
                let mut argv: Vec<String> = {
                    let argv: &[*mut libc::c_char; $c] = &*(args.unwrap() as *const [*mut i8; $c]);
                    let mut argv = argv
                    .to_vec()
                    .into_iter()
                    .map(|s|
                        std::ffi::CStr::from_ptr(s)
                        .to_string_lossy()
                        .trim_matches('\"')
                        .to_owned()
                    )
                    .collect::<Vec<String>>();
                    argv.reverse();
                    argv
                };
                #[allow(unused_variables, unused_mut)] // Caused by the 0 loop
                let mut c = 0;
                #[allow(unused_assignments, clippy::eval_order_dependence)]
                handle_output_and_return(
                    {
                        (self)(context, $(
                            if let Ok(val) = $param::from_arma(argv.pop().unwrap()) {
                                c += 1;
                                val
                            } else {
                                return format!("3{}", c).parse::<libc::c_int>().unwrap()
                            },
                        )*)
                    },
                    output,
                    size
                )
            }
        }
    }
});

unsafe fn handle_output_and_return<R>(
    ret: R,
    output: *mut libc::c_char,
    size: libc::size_t,
) -> libc::c_int
where
    R: IntoExtResult + 'static,
{
    let ret = ret.to_ext_result();
    let ok = ret.is_ok();
    if crate::write_cstr(
        {
            let value = match ret {
                Ok(x) | Err(x) => x,
            };
            match value {
                Value::String(s) => s,
                v => v.to_string(),
            }
        },
        output,
        size,
    )
    .is_none()
    {
        4
    } else if ok {
        0
    } else {
        9
    }
}

factory_tuple! { 0, }
factory_tuple! { 1, A }
factory_tuple! { 2, A B }
factory_tuple! { 3, A B C }
factory_tuple! { 4, A B C D }
factory_tuple! { 5, A B C D E }
factory_tuple! { 6, A B C D E F }
factory_tuple! { 7, A B C D E F G }
factory_tuple! { 8, A B C D E F G H }
factory_tuple! { 9, A B C D E F G H I }
factory_tuple! { 10, A B C D E F G H I J }
factory_tuple! { 11, A B C D E F G H I J K }
factory_tuple! { 12, A B C D E F G H I J K L }
