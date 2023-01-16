use core::fmt::*;

use crate::once::Once;

/// Implements [`Display`] by joining [`Iterator`] items with a separator
/// between each.
///
/// This may be used as a non-allocating alternative to
/// [`[T]::join()`](https://doc.rust-lang.org/std/primitive.slice.html#method.join)
/// or [`itertools::join()`](https://docs.rs/itertools/latest/itertools/fn.join.html).
///
/// If [`Clone`] for the [`Iterator`] is too expensive, consider using
/// [`join_once()`].
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

/// Implements [`Display`] by joining [`Iterator`] items with a separator
/// between each, at most once.
///
/// This is a non-[`Clone`] alternative to [`join()`]. It uses interior
/// mutability to take ownership of the iterator in the first call to
/// [`Display::fmt()`]. As a result, [`JoinOnce`] does not implement [`Sync`].
///
/// This is similar to [`Itertools::format()`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.format)
/// except that it will not panic if called more than once.
///
/// # Examples
///
/// ```
/// let value = fmty::join_once(["hola", "mundo"], " ");
/// assert_eq!(value.to_string(), "hola mundo");
///
/// assert_eq!(value.to_string(), "");
/// ```
pub fn join_once<I, S>(iter: I, sep: S) -> JoinOnce<I::IntoIter, S>
where
    I: IntoIterator,
{
    Join { iter: Once::new(iter.into_iter()), sep }
}

/// Implements [`Display`] by joining mapped [`Iterator`] results with a
/// separator between each.
///
/// Unlike <code>[join]\([iter.map(f)](Iterator::map), sep\)</code>, this
/// function does not require the mapping closure to be [`Clone`].
///
/// If [`Clone`] for the [`Iterator`] is too expensive, consider using
/// [`join_map_once()`].
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

/// Implements [`Display`] by joining mapped [`Iterator`] results with a
/// separator between each, at most once.
///
/// This is a non-[`Clone`] alternative to [`join_map()`]. It uses interior
/// mutability to take ownership of the iterator in the first call to
/// [`Display::fmt()`]. As a result, [`JoinMapOnce`] does not implement
/// [`Sync`].
///
/// # Examples
///
/// ```
/// let value = fmty::join_map_once(["hola", "mundo"], " ", fmty::to_uppercase);
/// assert_eq!(value.to_string(), "HOLA MUNDO");
///
/// assert_eq!(value.to_string(), "");
/// ```
pub fn join_map_once<I, S, R, F>(
    iter: I,
    sep: S,
    f: F,
) -> JoinMapOnce<I::IntoIter, S, F>
where
    I: IntoIterator,
    F: Fn(I::Item) -> R,
{
    JoinMap { iter: Once::new(iter.into_iter()), sep, map: f }
}

/// Implements [`Display`] by joining [`Iterator`] items with `, ` between each.
///
/// This is equivalent to <code>[join]\(iter, \", \"\)</code>.
///
/// If [`Clone`] for the [`Iterator`] is too expensive, consider using
/// [`csv_once()`].
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

/// Implements [`Display`] by joining [`Iterator`] items with `, ` between each,
/// at most once.
///
/// This is equivalent to <code>[join_once]\(iter, \", \"\)</code>.
///
/// This is a non-[`Clone`] alternative to [`csv()`]. It uses interior
/// mutability to take ownership of the iterator in the first call to
/// [`Display::fmt()`]. As a result, [`CsvOnce`] does not implement [`Sync`].
///
/// # Examples
///
/// ```
/// let value = fmty::csv_once(["hola", "mundo"]);
/// assert_eq!(value.to_string(), "hola, mundo");
///
/// assert_eq!(value.to_string(), "");
/// ```
pub fn csv_once<I>(iter: I) -> CsvOnce<I::IntoIter>
where
    I: IntoIterator,
{
    join_once(iter, ", ")
}

/// Implements [`Display`] by joining mapped [`Iterator`] results with `, `
/// between each.
///
/// Unlike <code>[csv]\([iter.map(f)](Iterator::map)\)</code>, this function
/// does not require the mapping closure to be [`Clone`].
///
/// If [`Clone`] for the [`Iterator`] is too expensive, consider using
/// [`csv_map_once()`].
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

/// Implements [`Display`] by joining mapped [`Iterator`] results with `, `
/// between each, at most once.
///
/// # Examples
///
/// ```
/// let value = fmty::csv_map_once(["hola", "mundo"], fmty::to_uppercase);
/// assert_eq!(value.to_string(), "HOLA, MUNDO");
///
/// assert_eq!(value.to_string(), "");
/// ```
pub fn csv_map_once<I, R, F>(iter: I, f: F) -> CsvMapOnce<I::IntoIter, F>
where
    I: IntoIterator,
    F: Fn(I::Item) -> R,
{
    join_map_once(iter, ", ", f)
}

/// See [`join()`].
#[derive(Clone, Copy)]
pub struct Join<I, S> {
    iter: I,
    sep: S,
}

/// See [`join_once()`].
pub type JoinOnce<I, S> = Join<Once<I>, S>;

/// See [`join_map()`].
#[derive(Clone, Copy)]
pub struct JoinMap<I, S, F> {
    iter: I,
    sep: S,
    map: F,
}

/// See [`join_map_once()`].
pub type JoinMapOnce<I, S, F> = JoinMap<Once<I>, S, F>;

/// See [`csv()`].
pub type Csv<I> = Join<I, &'static str>;

/// See [`csv_once()`].
pub type CsvOnce<I> = Csv<Once<I>>;

/// See [`csv_map()`].
pub type CsvMap<I, F> = JoinMap<I, &'static str, F>;

/// See [`csv_map_once()`].
pub type CsvMapOnce<I, F> = CsvMap<Once<I>, F>;

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

impl<I, S> Display for JoinOnce<I, S>
where
    I: Iterator,
    I::Item: Display,
    S: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(mut iter) = self.iter.take() {
            if let Some(item) = iter.next() {
                write!(f, "{item}")?;
            }

            for item in iter {
                write!(f, "{}{item}", self.sep)?;
            }
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

impl<I, S, F, R> Display for JoinMapOnce<I, S, F>
where
    I: Iterator,
    S: Display,
    F: Fn(I::Item) -> R,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(mut iter) = self.iter.take() {
            if let Some(item) = iter.next() {
                write!(f, "{}", (self.map)(item))?;
            }

            for item in iter {
                write!(f, "{}{}", self.sep, (self.map)(item))?;
            }
        }
        Ok(())
    }
}
