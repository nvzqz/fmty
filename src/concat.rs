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

/// Like [`concat()`] over fixed items that may be different types.
///
/// As an optimization, this macro may return its input as-is or [`&str`](str)
/// (if empty). Although this will not change in the future, it is good practice
/// to treat the output type as an opaque <code>impl [Display]</code> rather
/// than rely on the concrete type.
///
/// # Examples
///
/// ```
/// assert_eq!(fmty::concat!().to_string(),          "");
/// assert_eq!(fmty::concat!("hola").to_string(),    "hola");
/// assert_eq!(fmty::concat!("hola", 1).to_string(), "hola1");
/// ```
#[macro_export]
macro_rules! concat {
    () => { "" };
    ($x:expr $(,)?) => { $x };
    ($x:expr, $y:expr $(, $rest:expr)+ $(,)?) => {
        $crate::_priv::concat::Concat(($x, $y, $crate::concat!($($rest),+)))
    };
    ($x:expr $(, $rest:expr)+ $(,)?) => {
        $crate::_priv::concat::Concat(($x, $crate::concat!($($rest),+)))
    };
}

#[cfg(test)]
mod tests {
    use core::mem;

    #[test]
    fn concat3_size() {
        let value = crate::concat!('"', "hi", '"');

        assert_eq!(
            mem::size_of_val(&value),
            mem::size_of::<(char, &str, char)>()
        );
    }
}
