use core::fmt::*;

pub(crate) mod types {
    #[allow(unused)]
    use super::*;

    /// See [`fmt_with()`].
    #[derive(Clone, Copy)]
    pub struct FmtWith<F = fn(&mut Formatter) -> Result> {
        pub(super) fmt: F,
    }
}

use types::*;

/// Formats via a closure.
///
/// # Examples
///
/// ```
/// let value = Some("hola mundo");
///
/// let formatted = fmty::fmt_with(|f| {
///     if let Some(value) = value {
///         write!(f, "{value}")?;
///     }
///     Ok(())
/// });
///
/// assert_eq!(formatted.to_string(), "hola mundo");
/// ```
///
/// # Limitations
///
/// Values cannot be implicitly moved into an inner invocation of `fmt_with`
/// within `fmt_with`.
///
/// ```compile_fail
/// use fmty::fmt_with;
///
/// let s: String = "hola".to_owned();
///
/// let value = fmt_with(move |f| {
///     write!(f, "{}", fmt_with(move |f| {
///         write!(f, "{s}")
///     }))
/// });
/// # drop(value);
/// ```
///
/// To work around this, the inner value must be explicitly binded:
///
/// ```
/// # use fmty::fmt_with;
/// # let s: String = "hola".to_owned();
/// let inner = fmt_with(|f| write!(f, "{s}"));
/// let value = fmt_with(|f| write!(f, "{inner}"));
/// # drop(value);
/// ```
///
/// See [Rust issue #107623](https://github.com/rust-lang/rust/issues/107623)
/// for tracking the status of this limitation.
pub fn fmt_with<F: Fn(&mut Formatter) -> Result>(fmt: F) -> FmtWith<F> {
    fmt.into()
}

impl<F: Fn(&mut Formatter) -> Result> From<F> for FmtWith<F> {
    fn from(fmt: F) -> Self {
        Self { fmt }
    }
}

impl<F: Fn(&mut Formatter) -> Result> Debug for FmtWith<F> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        (self.fmt)(f)
    }
}

impl<F: Fn(&mut Formatter) -> Result> Display for FmtWith<F> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        (self.fmt)(f)
    }
}
