use crate::call_context::{ArmaContextManager, CallContext, CallContextStackTrace};
use crate::ext_result::IntoExtResult;
use crate::flags::FeatureFlags;
use crate::value::{FromArma, Value};
use crate::Context;

type HandlerFunc = Box<
    dyn Fn(
        Context,
        &ArmaContextManager,
        *mut libc::c_char,
        libc::size_t,
        Option<*mut *mut i8>,
        Option<libc::c_int>,
    ) -> libc::c_int,
>;

#[doc(hidden)]
/// A wrapper for `HandlerFunc`
pub struct Handler {
    /// The function to call
    pub handler: HandlerFunc,
}

#[doc(hidden)]
/// Create a new handler from a Factory
pub fn fn_handler<C, I, R>(command: C) -> Handler
where
    C: Factory<I, R> + 'static,
{
    Handler {
        handler: Box::new(
            move |context: Context,
                  acm: &ArmaContextManager,
                  output: *mut libc::c_char,
                  size: libc::size_t,
                  args: Option<*mut *mut i8>,
                  count: Option<libc::c_int>|
                  -> libc::c_int {
                unsafe { command.call(context, acm, output, size, args, count) }
            },
        ),
    }
}

#[doc(hidden)]
/// Execute a command
pub trait Executor: 'static {
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    unsafe fn call(
        &self,
        context: Context,
        acm: &ArmaContextManager,
        output: *mut libc::c_char,
        size: libc::size_t,
        args: Option<*mut *mut i8>,
        count: Option<libc::c_int>,
    );
}

#[doc(hidden)]
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
        acm: &ArmaContextManager,
        output: *mut libc::c_char,
        size: libc::size_t,
        args: Option<*mut *mut i8>,
        count: Option<libc::c_int>,
    ) -> libc::c_int;
}

macro_rules! execute {
    ($s:ident, $c:expr, $count:expr, $output:expr, $size:expr, $args:expr, ($( $vars:ident )*), ($( $param:ident, )*)) => {{
        let count = $count.unwrap_or_else(|| 0);
        if count != $c {
            return format!("2{}", count).parse::<libc::c_int>().unwrap();
        }
        if $c == 0 {
            handle_output_and_return(
                ($s)($( $vars, )* $($param::from_arma("".to_string()).unwrap(),)*),
                $output,
                $size
            )
        } else {
            #[allow(unused_variables, unused_mut)]
            let mut argv: Vec<String> = {
                let argv: &[*mut libc::c_char; $c] = &*($args.unwrap() as *const [*mut i8; $c]);
                let mut argv = argv
                    .to_vec()
                    .into_iter()
                    .map(|s| {
                        std::ffi::CStr::from_ptr(s).to_string_lossy().to_string()
                    })
                    .collect::<Vec<String>>();
                argv.reverse();
                argv
            };
            #[allow(unused_variables, unused_mut)] // Caused by the 0 loop
            let mut c = 0;
            #[allow(unused_assignments, clippy::mixed_read_write_in_expression)]
            handle_output_and_return(
                {
                    ($s)($( $vars, )* $(
                        if let Ok(val) = $param::from_arma(argv.pop().unwrap()) {
                            c += 1;
                            val
                        } else {
                            return format!("3{}", c).parse::<libc::c_int>().unwrap()
                        },
                    )*)
                },
                $output,
                $size
            )
        }
    }};
}

macro_rules! factory_tuple ({ $c: expr, $($param:ident)* } => {
    impl<$($param,)* ER> Executor for dyn Factory<($($param,)*), ER>
    where
        ER: 'static,
        $($param: FromArma + 'static,)*
    {
        unsafe fn call(
            &self,
            context: Context,
            acm: &ArmaContextManager,
            output: *mut libc::c_char,
            size: libc::size_t,
            args: Option<*mut *mut i8>,
            count: Option<libc::c_int>,
        ) {
            self.call(context, acm, output, size, args, count);
        }
    }

    // No context
    impl<Func, $($param,)* ER> Factory<($($param,)*), ER> for Func
    where
        ER: IntoExtResult + 'static,
        Func: Fn($($param),*) -> ER,
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, _: Context, _: &ArmaContextManager, output: *mut libc::c_char, size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int {
            let count = count.unwrap_or_else(|| 0);
            if count != $c {
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
                        .map(|s| {
                            std::ffi::CStr::from_ptr(s).to_string_lossy().to_string()
                        })
                        .collect::<Vec<String>>();
                    argv.reverse();
                    argv
                };
                #[allow(unused_variables, unused_mut)] // Caused by the 0 loop
                let mut c = 0;
                #[allow(unused_assignments, clippy::mixed_read_write_in_expression)]
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

    // Context
    impl<Func, $($param,)* ER> Factory<(Context, $($param,)*), ER> for Func
    where
        ER: IntoExtResult + 'static,
        Func: Fn(Context, $($param),*) -> ER,
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, context: Context, _: &ArmaContextManager, output: *mut libc::c_char, size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int {
            execute!(self, $c, count, output, size, args, (context), ($($param,)*))
        }
    }

    // Call Context
    impl<Func, $($param,)* ER> Factory<(CallContext, $($param,)*), ER> for Func
    where
        ER: IntoExtResult + 'static,
        Func: Fn(CallContext, $($param),*) -> ER,
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, _: Context, acm: &ArmaContextManager, output: *mut libc::c_char, size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int {
            crate::RVExtensionFeatureFlags = FeatureFlags::default().with_context_stack_trace(false).as_bits();
            let call_context = acm.request().into_without_stack();
            execute!(self, $c, count, output, size, args, (call_context), ($($param,)*))
        }
    }

    // Call Context with Stack Trace
    impl<Func, $($param,)* ER> Factory<(CallContextStackTrace, $($param,)*), ER> for Func
    where
        ER: IntoExtResult + 'static,
        Func: Fn(CallContextStackTrace, $($param),*) -> ER,
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, _: Context, acm: &ArmaContextManager, output: *mut libc::c_char, size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int {
            crate::RVExtensionFeatureFlags = FeatureFlags::default().with_context_stack_trace(true).as_bits();
            let call_context = acm.request();
            execute!(self, $c, count, output, size, args, (call_context), ($($param,)*))
        }
    }

    // Context & Call Context
    impl<Func, $($param,)* ER> Factory<(Context, CallContext, $($param,)*), ER> for Func
    where
        ER: IntoExtResult + 'static,
        Func: Fn(Context, CallContext, $($param),*) -> ER,
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, context: Context, acm: &ArmaContextManager, output: *mut libc::c_char, size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int {
            crate::RVExtensionFeatureFlags = FeatureFlags::default().with_context_stack_trace(false).as_bits();
            let call_context = acm.request().into_without_stack();
            execute!(self, $c, count, output, size, args, (context call_context), ($($param,)*))
        }
    }

    // Context & Call Context with Stack Trace
    impl<Func, $($param,)* ER> Factory<(Context, CallContextStackTrace, $($param,)*), ER> for Func
    where
        ER: IntoExtResult + 'static,
        Func: Fn(Context, CallContextStackTrace, $($param),*) -> ER,
        $($param: FromArma,)*
    {
        #[allow(non_snake_case)]
        unsafe fn call(&self, context: Context, acm: &ArmaContextManager, output: *mut libc::c_char, size: libc::size_t, args: Option<*mut *mut i8>, count: Option<libc::c_int>) -> libc::c_int {
            crate::RVExtensionFeatureFlags = FeatureFlags::default().with_context_stack_trace(true).as_bits();
            let call_context = acm.request();
            execute!(self, $c, count, output, size, args, (context call_context), ($($param,)*))
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
factory_tuple! { 13, A B C D E F G H I J K L M }
factory_tuple! { 14, A B C D E F G H I J K L M N }
factory_tuple! { 15, A B C D E F G H I J K L M N O }
factory_tuple! { 16, A B C D E F G H I J K L M N O P }
factory_tuple! { 17, A B C D E F G H I J K L M N O P Q }
factory_tuple! { 18, A B C D E F G H I J K L M N O P Q R }
factory_tuple! { 19, A B C D E F G H I J K L M N O P Q R S }
factory_tuple! { 20, A B C D E F G H I J K L M N O P Q R S T }
factory_tuple! { 21, A B C D E F G H I J K L M N O P Q R S T U }
factory_tuple! { 22, A B C D E F G H I J K L M N O P Q R S T U V }
factory_tuple! { 23, A B C D E F G H I J K L M N O P Q R S T U V W }
factory_tuple! { 24, A B C D E F G H I J K L M N O P Q R S T U V W X }
factory_tuple! { 25, A B C D E F G H I J K L M N O P Q R S T U V W X Y }
factory_tuple! { 26, A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }
