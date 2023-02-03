/// Like [`core::format_args!`], but with full ownership.
///
/// # Examples
///
/// ```
/// use std::fmt::Display;
///
/// fn format(a: String, b: String) -> impl Display {
///     fmty::format_args!("{a} + {b}")
/// }
///
/// let value = format(1.to_string(), 2.to_string());
/// assert_eq!(value.to_string(), "1 + 2");
/// ```
///
/// Like [`core::format_args!`], the result implements [`Debug`].
///
/// ```
/// # let (a, b) = (1, 2);
/// let value = fmty::format_args!("{a} + {b}");
/// let debug = format!("{:?}", value);
/// assert_eq!(debug, "1 + 2");
/// ```
///
/// This macro is also aliased as `fmt_args` for optional brevity.
///
/// ```
/// # let (a, b) = (1, 2);
/// fmty::fmt_args!("{a} + {b}")
/// # ;
/// ```
///
/// # Limitations
///
/// Because of how this macro is implemented, nested invocations cannot be implicitly owned.
///
/// ```compile_fail
/// use fmty::fmt_args;
///
/// let s: String = "hola".to_owned();
///
/// let value = fmt_args!("{}", fmt_args!("{}", s));
/// # drop(value);
/// ```
///
/// To work around this, the inner value must be explicitly binded:
///
/// ```
/// # use fmty::fmt_args;
/// # let s: String = "hola".to_owned();
/// let inner = fmt_args!("{}", s);
/// let value = fmt_args!("{}", inner);
/// # drop(value);
/// ```
///
/// Alternatively, use [`core::format_args!`] for the inner value. This works
/// because all arguments are lazily evaluated within a closure.
///
/// ```
/// # use fmty::fmt_args;
/// # let s: String = "hola".to_owned();
/// let value = fmt_args!("{}", format_args!("{}", s));
/// # drop(value);
/// ```
///
/// See [issue #1](https://github.com/nvzqz/fmty/issues/1) for tracking the
/// status of this limitation.
#[macro_export]
macro_rules! format_args {
    ($($tt:tt)+) => {
        $crate::fmt_with(move |__format_args_formatter__| {
            ::core::write!(__format_args_formatter__, $($tt)+)
        })
    };
}
