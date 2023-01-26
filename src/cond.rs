use crate::types::NoOp;
use core::fmt::{self, Debug, Display, Formatter};

pub(crate) mod types {
    #[allow(unused)]
    use super::*;

    /// See [`cond()`], [`cond_option()`].
    pub type Cond<T> = CondOr<T, NoOp>;

    /// See [`cond_or()`], [`cond_option_or()`], [`cond_result()`].
    #[derive(Clone, Copy)]
    pub struct CondOr<T, U = T> {
        pub(super) value: Result<T, U>,
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

/// Conditionally writes a value, or its fallback if `false`.
///
/// # Examples
///
/// ```
/// assert_eq!(fmty::cond_or(true,  "hola", "mundo").to_string(), "hola");
/// assert_eq!(fmty::cond_or(false, "hola", "mundo").to_string(), "mundo");
/// ```
pub fn cond_or<T, U>(write: bool, value: T, fallback: U) -> CondOr<T, U> {
    cond_result(if write { Ok(value) } else { Err(fallback) })
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
pub fn cond_option<T>(option: Option<T>) -> Cond<T> {
    cond_option_or(option, crate::no_op())
}

/// Conditionally writes an [`Option`], or its fallback if [`None`].
///
/// This is equivalent to
/// <code>[cond_result]\(option.[ok_or](Option::ok_or)(fallback)\)</code>.
///
/// If not using two different types, consider using [`Option::or()`] instead.
///
/// # Examples
///
/// ```
/// assert_eq!(fmty::cond_option(Some("hola")).to_string(), "hola");
/// assert_eq!(fmty::cond_option(None::<&str>).to_string(), "");
/// ```
pub fn cond_option_or<T, U>(option: Option<T>, fallback: U) -> CondOr<T, U> {
    cond_result(option.ok_or(fallback))
}

/// Conditionally writes a [`Result`] variant.
///
/// # Examples
///
/// ```
/// assert_eq!(fmty::cond_result::<_, &str>(Ok("hola")).to_string(),   "hola");
/// assert_eq!(fmty::cond_result::<&str, _>(Err("mundo")).to_string(), "mundo");
/// ```
pub fn cond_result<T, U>(result: Result<T, U>) -> CondOr<T, U> {
    CondOr { value: result }
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

impl<T: Debug, U: Debug> Debug for CondOr<T, U> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.value {
            Ok(value) => value.fmt(f),
            Err(value) => value.fmt(f),
        }
    }
}

impl<T: Display, U: Display> Display for CondOr<T, U> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.value {
            Ok(value) => value.fmt(f),
            Err(value) => value.fmt(f),
        }
    }
}

impl<F, R> Debug for CondWith<Option<F>>
where
    F: Fn() -> R,
    R: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(value) = (self.make_value)() {
            value.fmt(f)?;
        }
        Ok(())
    }
}
