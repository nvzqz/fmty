use core::fmt::*;

/// Repeats a value `n` times.
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

/// Repeats `n` results of a closure.
///
/// # Examples
///
/// ```
/// use std::cell::Cell;
///
/// let counter = Cell::new(1);
///
/// let value = fmty::repeat_with(3, || {
///     let result = counter.get();
///     counter.set(result + 1);
///     result
/// });
///
/// assert_eq!(value.to_string(), "123");
/// ```
pub fn repeat_with<F>(n: usize, f: F) -> RepeatWith<F> {
    RepeatWith { n, f }
}

/// See [`repeat()`].
#[derive(Clone, Copy)]
pub struct Repeat<T> {
    value: T,
    n: usize,
}

/// See [`repeat_with()`].
#[derive(Clone, Copy)]
pub struct RepeatWith<F> {
    // Although this could alias `Concat<iter::Take<iter::RepeatWith<F>>>`, it
    // would not be able to implement `Copy` because `Take` doesn't.
    f: F,
    n: usize,
}

impl<T: Debug> Debug for Repeat<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for _ in 0..self.n {
            write!(f, "{:?}", self.value)?;
        }
        Ok(())
    }
}

impl<T: Display> Display for Repeat<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for _ in 0..self.n {
            write!(f, "{}", self.value)?;
        }
        Ok(())
    }
}

impl<F, R> Debug for RepeatWith<F>
where
    F: Fn() -> R,
    R: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        for _ in 0..self.n {
            write!(f, "{:?}", (self.f)())?;
        }
        Ok(())
    }
}

impl<F, R> Display for RepeatWith<F>
where
    F: Fn() -> R,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        for _ in 0..self.n {
            write!(f, "{}", (self.f)())?;
        }
        Ok(())
    }
}
