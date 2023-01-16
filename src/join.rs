use core::{fmt::*, iter};

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

/// Implements [`Display`] by joining [`Iterator::map()`] results with a
/// separator between each.
///
/// # Examples
///
/// ```
/// let value = fmty::join_map(["hola", "mundo"], " ", fmty::to_uppercase);
/// assert_eq!(value.to_string(), "HOLA MUNDO");
/// ```
pub fn join_map<I, S, R, F>(iter: I, sep: S, f: F) -> JoinMap<I::IntoIter, S, F>
where
    I: IntoIterator,
    I::IntoIter: Clone,
    F: Fn(I::Item) -> R + Clone,
{
    join(iter.into_iter().map(f), sep)
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

/// Implements [`Display`] by joining [`Iterator::map()`] results with `, `
/// between each.
///
/// # Examples
///
/// ```
/// let value = fmty::csv_map(["hola", "mundo"], fmty::to_uppercase);
/// assert_eq!(value.to_string(), "HOLA, MUNDO");
/// ```
pub fn csv_map<I, R, F>(iter: I, f: F) -> CsvMap<I::IntoIter, F>
where
    I: IntoIterator,
    I::IntoIter: Clone,
    F: Fn(I::Item) -> R + Clone,
{
    csv(iter.into_iter().map(f))
}

/// See [`join()`].
#[derive(Clone, Copy)]
pub struct Join<I, S> {
    iter: I,
    sep: S,
}

/// See [`join_map()`].
pub type JoinMap<I, S, F> = Join<iter::Map<I, F>, S>;

/// See [`csv()`].
pub type Csv<I> = Join<I, &'static str>;

/// See [`csv_map()`].
pub type CsvMap<I, F> = Csv<iter::Map<I, F>>;

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
