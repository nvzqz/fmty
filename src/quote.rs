#[allow(unused)]
use core::fmt::Display;

use crate::{infix, Infix};

/// Implements [`Display`] by placing a value between `'`.
///
/// # Examples
///
/// ```
/// let value = fmty::quote_single(123);
///
/// assert_eq!(value.to_string(), "'123'");
/// ```
pub fn quote_single<T>(value: T) -> Infix<T, char> {
    infix('\'', value, '\'')
}

/// Implements [`Display`] by placing a value between `"`.
///
/// # Examples
///
/// ```
/// let value = fmty::quote_double(123);
///
/// assert_eq!(value.to_string(), "\"123\"");
/// ```
pub fn quote_double<T>(value: T) -> Infix<T, char> {
    infix('"', value, '"')
}

/// Implements [`Display`] by placing a value between <code>`</code>.
///
/// # Examples
///
/// ```
/// let value = fmty::quote_backtick("code");
///
/// assert_eq!(value.to_string(), "`code`");
/// ```
pub fn quote_backtick<T: Display>(value: T) -> Infix<T, char> {
    infix('`', value, '`')
}

/// Implements [`Display`] by placing a value between `‘` and `’`.
///
/// # Examples
///
/// ```
/// let value = fmty::quote_directed_single("supposedly");
///
/// assert_eq!(value.to_string(), "‘supposedly’");
/// ```
pub fn quote_directed_single<T>(value: T) -> Infix<T, char> {
    infix('‘', value, '’')
}

/// Implements [`Display`] by placing a value between `“` and `”`.
///
/// # Examples
///
/// ```
/// let value = fmty::quote_directed_double("Stunning!");
///
/// assert_eq!(value.to_string(), "“Stunning!”");
/// ```
pub fn quote_directed_double<T>(value: T) -> Infix<T, char> {
    infix('“', value, '”')
}

/// Implements [`Display`] by placing a value between [`‚` and `‘`](https://en.wikipedia.org/wiki/Quotation_mark#German).
///
/// # Examples
///
/// ```
/// let value = fmty::quote_low_single("Wie geht's?");
///
/// assert_eq!(value.to_string(), "‚Wie geht's?‘");
/// ```
pub fn quote_low_single<T>(value: T) -> Infix<T, char> {
    infix('‚', value, '‘')
}

/// Implements [`Display`] by placing a value between [`„` and `“`](https://en.wikipedia.org/wiki/Quotation_mark#German).
///
/// # Examples
///
/// ```
/// let value = fmty::quote_low_double("Perfekt!");
///
/// assert_eq!(value.to_string(), "„Perfekt!“");
/// ```
pub fn quote_low_double<T>(value: T) -> Infix<T, char> {
    infix('„', value, '“')
}

/// Implements [`Display`] by placing a value between [`‹` and `›`](https://en.wikipedia.org/wiki/Guillemet).
///
/// # Examples
///
/// ```
/// let value = fmty::quote_guillemet_single("Comment ça va?");
///
/// assert_eq!(value.to_string(), "‹Comment ça va?›");
/// ```
pub fn quote_guillemet_single<T>(value: T) -> Infix<T, char> {
    infix('‹', value, '›')
}

/// Implements [`Display`] by placing a value between [`«` and `»`](https://en.wikipedia.org/wiki/Guillemet).
///
/// # Examples
///
/// ```
/// let value = fmty::quote_guillemet_double("Impressionnante!");
///
/// assert_eq!(value.to_string(), "«Impressionnante!»");
/// ```
pub fn quote_guillemet_double<T>(value: T) -> Infix<T, char> {
    infix('«', value, '»')
}
