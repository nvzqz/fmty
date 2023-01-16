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

/// Implements [`Display`] by joining mapped [`Iterator`] results with a
/// separator between each.
///
/// Unlike <code>[join]\([iter.map(f)](Iterator::map), sep\)</code>, this
/// function does not require the mapping closure to be [`Clone`].
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
    F: Fn(I::Item) -> R,
{
    JoinMap { iter: iter.into_iter(), sep, map: f }
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

/// Implements [`Display`] by joining mapped [`Iterator`] results with `, `
/// between each.
///
/// Unlike <code>[csv]\([iter.map(f)](Iterator::map)\)</code>, this function
/// does not require the mapping closure to be [`Clone`].
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
    F: Fn(I::Item) -> R,
{
    join_map(iter, ", ", f)
}

/// See [`join()`].
#[derive(Clone, Copy)]
pub struct Join<I, S> {
    iter: I,
    sep: S,
}

/// See [`join_map()`].
#[derive(Clone, Copy)]
pub struct JoinMap<I, S, F> {
    iter: I,
    sep: S,
    map: F,
}

/// See [`csv()`].
pub type Csv<I> = Join<I, &'static str>;

/// See [`csv_map()`].
pub type CsvMap<I, F> = JoinMap<I, &'static str, F>;

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

impl<I, S, F, R> Display for JoinMap<I, S, F>
where
    I: Iterator + Clone,
    S: Display,
    F: Fn(I::Item) -> R,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut iter = self.iter.clone();

        if let Some(item) = iter.next() {
            write!(f, "{}", (self.map)(item))?;
        }

        for item in iter {
            write!(f, "{}{}", self.sep, (self.map)(item))?;
        }

        Ok(())
    }
}
