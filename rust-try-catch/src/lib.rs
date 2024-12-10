#![deny(missing_docs)]

#![doc = include_str!("../../../../README.md")]

use std::any::Any;
use std::backtrace::Backtrace;
use std::panic::AssertUnwindSafe;

#[cfg(not(panic = "unwind"))]
compile_error!("Try catch only works when panic = \"unwind\"");

/// A flexible `try_catch` macro that provides structured error and panic handling
/// with an optional `finally` block for cleanup.
///
/// # Description
/// The `try_catch!` macro allows you to define a `try` block for executing code, followed by multiple `catch`
/// blocks for handling exceptions or panics. Additionally, a `finally` block can be specified for cleanup actions.
///
/// # Syntax
/// ```text
/// try_catch! {
///     try {
///         <try_block>
///     } catch (<exception_pattern> => <exception_type>) {
///         <catch_block>
///     } catch exception (<catch_all_pattern>) {
///         <catch_all_block>
///     } catch panic (<panic_pattern>) {
///         <catch_panic_block>
///     } finally {
///         <finally_block>
///     }
/// }
/// ```
///
/// - **try block**: The primary code to execute.
/// - **catch blocks**: Handle exceptions matching specific types.
/// - **catch exception block** (optional): A generic handler for exceptions not caught by specific `catch` blocks.
/// - **catch panic block** (optional): Handle non-exception panics.
/// - **finally block** (optional): Executes unconditionally after the `try` block and any `catch` blocks.
///
/// ## Notes
///  - at least 2 blocks have to be defined you cannot have a bare try {} expression
///  - the try and catch blocks should both return the same type
///  - the finally block should return () aka "unit"
///
/// # Features
///
/// - Matches exceptions based on type.
/// - Catches generic exceptions with `catch exception`.
/// - Handles panics using `catch panic`.
/// - Ensures cleanup via an optional `finally` block.
///
/// # Usage
///
/// ## Handling specific exceptions
/// ```
/// #[derive(Debug)]
/// struct MyErrorType;
///
/// fn some_function() -> Result<i32, MyErrorType> {
///     Err(MyErrorType)
/// }
///
/// let result = rust_try_catch::try_catch! {
///     try {
///         rust_try_catch::tri!(some_function())
///     } catch (e => MyErrorType) {
///         println!("Caught MyErrorType: {e:?}");
///         -1
///     }
/// };
/// assert_eq!(result, -1);
/// ```
///
/// ## Catching all exceptions
/// ```
/// # fn another_function() {
/// #     rust_try_catch::throw("Haha I failed");
/// # }
///
/// let result = rust_try_catch::try_catch! {
///     try {
///         // Code that might throw.
///         another_function();
///         0
///     } catch exception (e) {
///         println!("Caught an exception: {:?}", e);
///         -2
///     }
/// };
/// assert_eq!(result, -2);
/// ```
///
/// ## Handling panics
/// ```
/// let result = rust_try_catch::try_catch! {
///     try {
///         // Code that might panic.
///         panic!("Unexpected error");
///         0
///     } catch panic (e) {
///         println!("Caught a panic: {:?}", e);
///         -101
///     }
/// };
/// assert_eq!(result, -101);
/// ```
///
/// ## Using a finally block
/// ```
/// let mut cleanup = false;
/// let result = rust_try_catch::try_catch! {
///     try {
///         // Code execution.
///         42
///     } finally {
///         cleanup = true;
///     }
/// };
/// assert_eq!(result, 42);
/// assert!(cleanup);
/// ```
///
/// ## Combining handlers
/// ```
/// # #[derive(Debug)]
/// # struct SpecificError;
/// # let risky_operation = || rust_try_catch::throw(SpecificError);
///
/// let result = rust_try_catch::try_catch! {
///     try {
///         // Code execution.
///         risky_operation();
///         0
///     } catch (e => SpecificError) {
///         println!("Caught SpecificError: {e:?}");
///         -1
///     } catch exception (e) {
///         println!("Caught general exception: {e:?}");
///         -2
///     } catch panic (e) {
///         println!("Caught a panic: {e:?}");
///         -3
///     } finally {
///         println!("Cleanup actions here.");
///     }
/// };
/// ```
///
/// # Notes
///
/// - The `catch panic` block is only invoked for panics unrelated to exceptions handled by the macro.
/// - The `finally` block runs regardless of whether an exception or panic occurred.
/// - Unhandled exceptions or panics will propagate out of the macro.
///
/// # Examples
///
/// ## No exception or panic (doesn't compile)
/// ```compile_fail
/// let result = rust_try_catch::try_catch! {
///     try {
///         100
///     }
/// };
/// assert_eq!(result, 100);
/// ```
///
/// ## Exception without panic
/// ```
/// # use rust_try_catch::throw;
///
/// let result = rust_try_catch::try_catch! {
///     try {
///         throw("An error occurred");
///     } catch (e => &'static str) {
///         println!("Handled error: {}", e);
///         0
///     }
/// };
/// assert_eq!(result, 0);
/// ```
///
/// ## Panic recovery
/// ```
/// let result = rust_try_catch::try_catch! {
///     try {
///         panic!("Something went wrong!");
///     } catch panic (e) {
///         println!("Recovered from panic: {:?}", e);
///         1
///     }
/// };
/// assert_eq!(result, 1);
/// ```
#[macro_export]
macro_rules! try_catch {
    {
        try {
            $($try_body: tt)*
        } $(catch ($exception_name: pat => $exception_ty:ty) {
            $($catch_body: tt)*
        })* $(catch exception ($catch_all_exception_name: pat) {
            $($catch_all_exception_body: tt)*
        })? $(catch panic ($catch_panic_exception_name: pat) {
            $($catch_panic_exception_body: tt)*
        })? $(finally {
            $($finally_body: tt)*
        })?
    } => {{
        const {
            let count = $crate::__count_blocks!(
                {$($try_body)*}
                $({$exception_name})*
                $({$catch_all_exception_name})?
                $({$catch_panic_exception_name})?
                $({$($finally_body)*})?
            );

            if count < 2 {
                ::core::panic!("Using try {{ /*code*/ }} is equivalent to a no-op")
            }
        }

        struct FinallyDo<F: ::core::ops::FnOnce() -> ()>(::core::mem::ManuallyDrop<F>);
        impl<F: ::core::ops::FnOnce()> Drop for FinallyDo<F> {
            fn drop(&mut self) {
                (unsafe { ::core::mem::ManuallyDrop::take(&mut self.0) })()
            }
        }

        $(let _finally_guard = FinallyDo(::core::mem::ManuallyDrop::new(|| {
            $($finally_body)*
        }));)?

        let fun = ::std::panic::AssertUnwindSafe(|| { $($try_body)* });
        let val = match ::std::panic::catch_unwind(fun) {
            Ok(res) => res,
            Err(panic_payload) => 'ret_from_err: {
                let mut exception = match panic_payload.downcast::<$crate::Thrown>() {
                    Ok(box_thrown) => box_thrown,
                    Err(normal_panic) => {
                        $({
                            let $catch_panic_exception_name = normal_panic;
                            break 'ret_from_err ({$($catch_panic_exception_body)*})
                        })?
                        #[allow(unreachable_code)]
                        ::std::panic::resume_unwind(normal_panic)
                    }
                };

                $(
                    match exception.source.downcast::<$exception_ty>() {
                        Ok(box_error) => {
                            let $exception_name: $exception_ty = *box_error;

                            break 'ret_from_err ({
                               $($catch_body)*
                            })
                        }
                        Err(other_error) => exception.source = other_error,
                    }
                )*

                $({
                    let $catch_all_exception_name = exception.source;
                    break 'ret_from_err ({$($catch_all_exception_body)*})
                })?

                #[allow(unreachable_code)]
                ::std::panic::resume_unwind(exception)
            }
        };

        val
    }};
}

/// Unwraps a result or propagates its error as an exception.
///
/// tri! matches the given Result.
/// In case of the Ok variant, the expression has the value of the wrapped value.
/// In case of the Err variant, it retrieves the inner error, and calls throw on it.
#[macro_export]
macro_rules! tri {
    ($expr: expr) => {
        match ($expr) {
            ::core::result::Result::Ok(val) => val,
            ::core::result::Result::Err(err) => $crate::throw(err),
        }
    };
}

#[doc(hidden)]
pub struct Thrown {
    pub source: Box<dyn Any + Send>,
    pub type_name: &'static str,
    pub backtrace: Backtrace
}

/// Calling throw always results in a panic
/// 
/// for proper usage users must ensure that there is a function annotated with `rust_try_catch::throw_guard`
/// up in the call chain
pub fn throw<T: Any + Send + 'static>(x: T) -> ! {
    std::panic::resume_unwind(Box::new(Thrown {
        source: Box::new(x),
        type_name: std::any::type_name::<T>(),
        backtrace: Backtrace::force_capture()
    }))
}

/// # Description
/// wraps a function or closure, to prevent a thrown exception from propagating beyond them
/// and turns unhandled exceptions to a panic
///
/// # Note
/// thrown exceptions do not trigger the panic hook so if this isn't in the call chain before some code
/// throws, the process might exit abruptly due to a panic with an unspecified load
pub use rust_try_catch_macros::{throw_guard, closure_throw_guard};


#[doc(hidden)]
#[track_caller]
pub fn __throw_driver<T>(main: impl FnOnce() -> T) -> T {
    // help reduce size of throw_driver
    #[inline(never)]
    fn inner(f: &mut dyn FnMut()) {
        if let Err(panic) = std::panic::catch_unwind(AssertUnwindSafe(f)) {
            if let Some(Thrown { type_name, backtrace, .. }) = panic.downcast_ref() {
                panic!("unhandled exception {type_name} at {backtrace}");
            }

            std::panic::resume_unwind(panic)
        }
    }
    
    let mut main = Some(main);
    let mut output = None;
    inner(&mut || {
        // Safety: inner runs `f` at most once
        unsafe {
            let main_fn = main.take().unwrap_unchecked();
            // help skip destructor call
            std::hint::assert_unchecked(output.is_none());
            output = Some(main_fn())
        }
    });

    // Safety: if inner returns that means the closure ran to completion
    unsafe { output.unwrap_unchecked() }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __count_blocks {
    () => { 0 };
    ({$($tt:tt)*} $($rest:tt)*) => {
        1 + $crate::__count_blocks!($($rest)*)
    }
}