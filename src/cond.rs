use core::fmt::*;

pub(crate) mod types {
    #[allow(unused)]
    use super::*;

    /// See [`cond()`], [`cond_option()`].
    #[derive(Clone, Copy)]
    pub struct Cond<T> {
        pub(super) value: Option<T>,
    }

    /// See [`cond_with()`], [`cond_with_option()`].
    #[derive(Clone, Copy)]
    pub struct CondWith<F> {
        pub(super) make_value: F,
    }
}

use types::*;

/// Conditionally writes a value.
///
/// # Examples
///
/// ```
/// assert_eq!(fmty::cond(true,  "hola").to_string(), "hola");
/// assert_eq!(fmty::cond(false, "hola").to_string(), "");
/// ```
pub fn cond<T>(write: bool, value: T) -> Cond<T> {
    cond_option(if write { Some(value) } else { None })
}

/// Conditionally writes an [`Option`].
///
/// This is has the same behavior as
/// <code>[concat](crate::concat())(option)</code>
/// without the [`Clone`] requirement.
///
/// # Examples
///
/// ```
/// assert_eq!(fmty::cond_option(Some("hola")).to_string(), "hola");
/// assert_eq!(fmty::cond_option(None::<&str>).to_string(), "");
/// ```
pub fn cond_option<T>(value: Option<T>) -> Cond<T> {
    Cond { value }
}

/// Conditionally writes a closure result.
///
/// # Examples
///
/// ```
/// assert_eq!(fmty::cond_with(true,  || "hola").to_string(), "hola");
/// assert_eq!(fmty::cond_with(false, || "hola").to_string(), "");
/// ```
pub fn cond_with<R, F>(write: bool, f: F) -> CondWith<Option<F>>
where
    F: Fn() -> R,
{
    CondWith { make_value: if write { Some(f) } else { None } }
}

/// Conditionally writes a closure [`Option`] result.
///
/// # Examples
///
/// ```
/// assert_eq!(fmty::cond_with_option(|| Some("hola")).to_string(), "hola");
/// assert_eq!(fmty::cond_with_option(|| None::<&str>).to_string(), "");
/// ```
pub fn cond_with_option<R, F>(f: F) -> CondWith<F>
where
    F: Fn() -> Option<R>,
{
    CondWith { make_value: f }
}

impl<T: Debug> Debug for Cond<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(value) = &self.value {
            value.fmt(f)?;
        }
        Ok(())
    }
}

impl<T: Display> Display for Cond<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(value) = &self.value {
            value.fmt(f)?;
        }
        Ok(())
    }
}

impl<F, R> Debug for CondWith<Option<F>>
where
    F: Fn() -> R,
    R: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(make_value) = &self.make_value {
            make_value().fmt(f)?;
        }
        Ok(())
    }
}

impl<F, R> Display for CondWith<Option<F>>
where
    F: Fn() -> R,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(make_value) = &self.make_value {
            make_value().fmt(f)?;
        }
        Ok(())
    }
}

impl<F, R> Debug for CondWith<F>
where
    F: Fn() -> Option<R>,
    R: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(value) = (self.make_value)() {
            value.fmt(f)?;
        }
        Ok(())
    }
}

impl<F, R> Display for CondWith<F>
where
    F: Fn() -> Option<R>,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(value) = (self.make_value)() {
            value.fmt(f)?;
        }
        Ok(())
    }
}
