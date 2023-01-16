use core::fmt::*;

/// Implements [`Display`] by repeating a value `n` times.
///
/// This is a non-allocating alternative to
/// [`[T]::repeat()`](https://doc.rust-lang.org/std/primitive.slice.html#method.repeat) or
/// [`str::repeat()`](https://doc.rust-lang.org/std/primitive.str.html#method.repeat).
///
/// # Examples
///
/// ```
/// let value = fmty::repeat("123", 3);
/// assert_eq!(value.to_string(), "123123123");
/// ```
pub fn repeat<T>(value: T, n: usize) -> Repeat<T> {
    Repeat { value, n }
}

/// See [`repeat()`].
#[derive(Clone, Copy)]
pub struct Repeat<T> {
    value: T,
    n: usize,
}

impl<T: Display> Display for Repeat<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for _ in 0..self.n {
            write!(f, "{}", self.value)?;
        }
        Ok(())
    }
}
