use core::fmt::*;

/// Implements [`Display`] by concatenating [`Iterator`] items.
///
/// This is a non-allocating alternative to
/// [`[T]::concat()`](https://doc.rust-lang.org/std/primitive.slice.html#method.concat).
///
/// # Examples
///
/// ```
/// let value = fmty::concat(["hola", "mundo"]);
/// assert_eq!(value.to_string(), "holamundo");
/// ```
///
/// This can also be used to conditionally write [`Option`] because it
/// implements [`IntoIterator`].
///
/// ```
/// assert_eq!(fmty::concat(Some("hola")).to_string(), "hola");
/// assert_eq!(fmty::concat(None::<&str>).to_string(), "");
/// ```
pub fn concat<I>(iter: I) -> Concat<I::IntoIter>
where
    I: IntoIterator,
    I::IntoIter: Clone,
{
    iter.into()
}

/// See [`concat()`].
#[derive(Clone, Copy)]
pub struct Concat<I> {
    iter: I,
}

impl<I> From<I> for Concat<I::IntoIter>
where
    I: IntoIterator,
    I::IntoIter: Clone,
{
    fn from(iter: I) -> Self {
        Self { iter: iter.into_iter() }
    }
}

impl<I> Display for Concat<I>
where
    I: Iterator + Clone,
    I::Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        for item in self.iter.clone() {
            write!(f, "{item}")?;
        }
        Ok(())
    }
}
