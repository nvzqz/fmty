use core::fmt::*;

/// Implements [`Display`] by joining [`Iterator`] items with a separator
/// between each.
///
/// This may be used as a non-allocating alternative to
/// [`[T]::join()`](https://doc.rust-lang.org/std/primitive.slice.html#method.join)
/// or [`itertools::join()`](https://docs.rs/itertools/latest/itertools/fn.join.html).
///
/// # Examples
///
/// ```
/// let value = fmty::join(["hola", "mundo"], " ");
/// assert_eq!(value.to_string(), "hola mundo");
/// ```
pub fn join<I, S>(iter: I, sep: S) -> Join<I::IntoIter, S>
where
    I: IntoIterator,
    I::IntoIter: Clone,
{
    Join { iter: iter.into_iter(), sep }
}

/// Implements [`Display`] by joining [`Iterator`] items with `, ` between each.
///
/// This is equivalent to <code>[join]\(iter, \", \"\)</code>.
///
/// # Examples
///
/// ```
/// let value = fmty::csv(["hola", "mundo"]);
/// assert_eq!(value.to_string(), "hola, mundo");
/// ```
pub fn csv<I>(iter: I) -> Csv<I::IntoIter>
where
    I: IntoIterator,
    I::IntoIter: Clone,
{
    join(iter, ", ")
}

/// See [`join()`].
#[derive(Clone, Copy)]
pub struct Join<I, S> {
    iter: I,
    sep: S,
}

/// See [`csv()`].
pub type Csv<I> = Join<I, &'static str>;

impl<I, S> Display for Join<I, S>
where
    I: Iterator + Clone,
    I::Item: Display,
    S: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut iter = self.iter.clone();

        if let Some(item) = iter.next() {
            write!(f, "{item}")?;
        }

        for item in iter {
            write!(f, "{}{item}", self.sep)?;
        }

        Ok(())
    }
}
