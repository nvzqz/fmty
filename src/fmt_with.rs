use core::fmt::*;

/// Implements [`Display`] via a closure.
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
pub fn fmt_with<F: Fn(&mut Formatter) -> Result>(fmt: F) -> FmtWith<F> {
    fmt.into()
}

/// See [`fmt_with()`].
#[derive(Clone, Copy)]
pub struct FmtWith<F = fn(&mut Formatter) -> Result> {
    fmt: F,
}

impl<F: Fn(&mut Formatter) -> Result> From<F> for FmtWith<F> {
    fn from(fmt: F) -> Self {
        Self { fmt }
    }
}

impl<F: Fn(&mut Formatter) -> Result> Display for FmtWith<F> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        (self.fmt)(f)
    }
}
