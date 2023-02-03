/// Like [`format_args!`](crate::format_args!) or [`fmt_with()`](crate::fmt_with())
/// depending on usage.
///
/// This macro accepts either formatting arguments or a closure. It is similar
/// to [`format::lazy_format!`](https://docs.rs/format/0.2.*/format/macro.lazy_format.html)
/// with the added benefit of being a declarative macro rather than a procedural
/// macro.
///
/// This always returns
/// <code>[FmtWith]<impl [Fn]\(&mut [Formatter]\) -> [Result]></code>.
///
/// [FmtWith]: crate::types::FmtWith
/// [Formatter]: core::fmt::Formatter
/// [Result]: core::fmt::Result
///
/// # Examples
///
/// ```
/// let a = fmty::fmt!("hola");
/// let b = fmty::fmt!("{}", "hola");
/// let c = fmty::fmt!(|f| f.write_str("hola"));
///
/// assert_eq!(a.to_string(), b.to_string());
/// assert_eq!(a.to_string(), c.to_string());
/// ```
///
/// # Limitations
///
/// This has the same [limitations of `format_args!`](crate::format_args!#limitations)
/// and [limitations of `fmt_with()`](crate::fmt_with()#limitations).
#[macro_export]
macro_rules! fmt {
    ($fmt:literal $($args:tt)*) => {
        $crate::format_args!($fmt $($args)*)
    };
    ($fn:expr) => {
        $crate::fmt_with($fn)
    };
}
