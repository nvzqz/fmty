use core::fmt::*;

use crate::once::Once;

pub(crate) mod types {
    #[allow(unused)]
    use super::*;

    /// See [`concat()`].
    #[derive(Clone, Copy)]
    pub struct Concat<I> {
        pub(super) iter: I,
    }

    /// See [`concat_once()`].
    pub type ConcatOnce<I> = Concat<Once<I>>;

    /// See [`concat_map()`].
    #[derive(Clone, Copy)]
    pub struct ConcatMap<I, F> {
        pub(super) iter: I,
        pub(super) map: F,
    }

    /// See [`concat_map_once()`].
    pub type ConcatMapOnce<I, F> = ConcatMap<Once<I>, F>;

    /// See [`concat_tuple()`].
    #[derive(Clone, Copy)]
    pub struct ConcatTuple<T>(pub(super) T);
}

use types::*;

/// Concatenates [`Iterator`] items.
///
/// This is a non-allocating alternative to
/// [`[T]::concat()`](https://doc.rust-lang.org/std/primitive.slice.html#method.concat).
///
/// If [`Clone`] for the [`Iterator`] is too expensive, consider using
/// [`concat_once()`].
///
/// Although this can be used to conditionally write an [`Option`] (because it
/// implements [`IntoIterator`]), consider using [`cond_option()`](crate::cond_option)
/// for this instead.
///
/// # Examples
///
/// ```
/// let value = fmty::concat(["hola", "mundo"]);
/// assert_eq!(value.to_string(), "holamundo");
/// ```
pub fn concat<I>(iter: I) -> Concat<I::IntoIter>
where
    I: IntoIterator,
    I::IntoIter: Clone,
{
    iter.into()
}

/// Concatenates [`Iterator`] items, at most once.
///
/// This is a non-[`Clone`] alternative to [`concat()`]. It uses interior
/// mutability to take ownership of the iterator in the first call to
/// [`Display::fmt()`]. As a result, [`ConcatOnce`] does not implement [`Sync`].
///
/// # Examples
///
/// ```
/// let value = fmty::concat_once(["hola", "mundo"]);
/// assert_eq!(value.to_string(), "holamundo");
///
/// assert_eq!(value.to_string(), "");
/// ```
pub fn concat_once<I: IntoIterator>(iter: I) -> ConcatOnce<I::IntoIter> {
    Concat { iter: Once::new(iter.into_iter()) }
}

/// Concatenates mapped [`Iterator`] results.
///
/// Unlike <code>[concat](concat())\([iter.map(f)](Iterator::map)\)</code>, this
/// function does not require the mapping closure to be [`Clone`].
///
/// If [`Clone`] for the [`Iterator`] is too expensive, consider using
/// [`concat_map_once()`].
///
/// # Examples
///
/// ```
/// let value = fmty::concat_map(["hola", "mundo"], fmty::to_uppercase);
/// assert_eq!(value.to_string(), "HOLAMUNDO");
/// ```
pub fn concat_map<I, R, F>(iter: I, f: F) -> ConcatMap<I::IntoIter, F>
where
    I: IntoIterator,
    I::IntoIter: Clone,
    F: Fn(I::Item) -> R,
{
    ConcatMap { iter: iter.into_iter(), map: f }
}

/// Concatenates mapped [`Iterator`] results, at most once.
///
/// This is a non-[`Clone`] alternative to [`concat_map()`]. It uses interior
/// mutability to take ownership of the iterator in the first call to
/// [`Display::fmt()`]. As a result, [`ConcatMapOnce`] does not implement
/// [`Sync`].
///
/// # Examples
///
/// ```
/// let value = fmty::concat_map_once(["hola", "mundo"], fmty::to_uppercase);
/// assert_eq!(value.to_string(), "HOLAMUNDO");
///
/// assert_eq!(value.to_string(), "");
/// ```
pub fn concat_map_once<I, R, F>(iter: I, f: F) -> ConcatMapOnce<I::IntoIter, F>
where
    I: IntoIterator,
    F: Fn(I::Item) -> R,
{
    ConcatMap { iter: Once::new(iter.into_iter()), map: f }
}

/// Concatenates [tuple](prim@tuple) items that may be different types.
///
/// This function is limited to tuples of length 12. Consider using
/// [`concat!`](crate::concat!) if this limit is too low.
///
/// # Examples
///
/// ```
/// let value = fmty::concat_tuple(("hola", "mundo"));
/// assert_eq!(value.to_string(), "holamundo");
/// ```
pub fn concat_tuple<T>(tuple: T) -> ConcatTuple<T> {
    ConcatTuple(tuple)
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

impl<I> Debug for Concat<I>
where
    I: Iterator + Clone,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        for item in self.iter.clone() {
            write!(f, "{:?}", item)?;
        }
        Ok(())
    }
}

impl<I> Display for Concat<I>
where
    I: Iterator + Clone,
    I::Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        for item in self.iter.clone() {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl<I, F, R> Debug for ConcatMap<I, F>
where
    I: Iterator + Clone,
    F: Fn(I::Item) -> R,
    R: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        for item in self.iter.clone() {
            write!(f, "{:?}", (self.map)(item))?;
        }
        Ok(())
    }
}

impl<I, F, R> Display for ConcatMap<I, F>
where
    I: Iterator + Clone,
    F: Fn(I::Item) -> R,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        for item in self.iter.clone() {
            write!(f, "{}", (self.map)(item))?;
        }
        Ok(())
    }
}

impl<I> Debug for ConcatOnce<I>
where
    I: Iterator,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(iter) = self.iter.take() {
            for item in iter {
                write!(f, "{:?}", item)?;
            }
        }
        Ok(())
    }
}

impl<I> Display for ConcatOnce<I>
where
    I: Iterator,
    I::Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(iter) = self.iter.take() {
            for item in iter {
                write!(f, "{}", item)?;
            }
        }
        Ok(())
    }
}

impl<I, F, R> Debug for ConcatMapOnce<I, F>
where
    I: Iterator + Clone,
    F: Fn(I::Item) -> R,
    R: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(iter) = self.iter.take() {
            for item in iter {
                write!(f, "{:?}", (self.map)(item))?;
            }
        }
        Ok(())
    }
}

impl<I, F, R> Display for ConcatMapOnce<I, F>
where
    I: Iterator + Clone,
    F: Fn(I::Item) -> R,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(iter) = self.iter.take() {
            for item in iter {
                write!(f, "{}", (self.map)(item))?;
            }
        }
        Ok(())
    }
}

impl Debug for ConcatTuple<()> {
    #[inline]
    fn fmt(&self, _: &mut Formatter) -> Result {
        Ok(())
    }
}

impl Display for ConcatTuple<()> {
    #[inline]
    fn fmt(&self, _: &mut Formatter) -> Result {
        Ok(())
    }
}

/// Implements `Debug`/`Display` for `ConcatTuple<(T, ...)>`.
macro_rules! impl_tuple {
    () => {};
    ($($x:ident),+) => {
        impl<$($x),+> Debug for ConcatTuple<($($x,)+)>
        where
            $($x: Debug),+
        {
            fn fmt(&self, f: &mut Formatter) -> Result {
                #[allow(non_snake_case)]
                let ($($x,)+) = &self.0;
                write!(
                    f,
                    core::concat!($("{", core::stringify!($x), ":?}",)+),
                    $($x = $x),+
                )
            }
        }

        impl<$($x),+> Display for ConcatTuple<($($x,)+)>
        where
            $($x: Display),+
        {
            fn fmt(&self, f: &mut Formatter) -> Result {
                #[allow(non_snake_case)]
                let ($($x,)+) = &self.0;
                write!(
                    f,
                    core::concat!($("{", core::stringify!($x), "}",)+),
                    $($x = $x),+
                )
            }
        }

        peel!(impl_tuple: $($x),+);
    };
}

impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);

/// Concatenates items that may be different types.
///
/// This is like [`concat_tuple()`] but with no length limit.
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
    ($t0:expr $(,)?) => { $t0 };
    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, $t5:expr, $t6:expr, $t7:expr, $t8:expr, $t9:expr, $t10:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $crate::concat!($($rest),+)))
    };
    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, $t5:expr, $t6:expr, $t7:expr, $t8:expr, $t9:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $crate::concat!($($rest),+)))
    };
    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, $t5:expr, $t6:expr, $t7:expr, $t8:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $crate::concat!($($rest),+)))
    };
    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, $t5:expr, $t6:expr, $t7:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $t2, $t3, $t4, $t5, $t6, $t7, $crate::concat!($($rest),+)))
    };
    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, $t5:expr, $t6:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $t2, $t3, $t4, $t5, $t6, $crate::concat!($($rest),+)))
    };
    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, $t5:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $t2, $t3, $t4, $t5, $crate::concat!($($rest),+)))
    };
    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $t2, $t3, $t4, $crate::concat!($($rest),+)))
    };
    ($t0:expr, $t1:expr, $t2:expr, $t3:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $t2, $t3, $crate::concat!($($rest),+)))
    };
    ($t0:expr, $t1:expr, $t2:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $t2, $crate::concat!($($rest),+)))
    };
    ($t0:expr, $t1:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $t1, $crate::concat!($($rest),+)))
    };
    ($t0:expr $(, $rest:expr)+ $(,)?) => {
        $crate::concat_tuple(($t0, $crate::concat!($($rest),+)))
    };
}

#[cfg(test)]
mod tests {
    use alloc::{
        rc::{Rc, Weak},
        string::ToString,
    };
    use core::{iter, mem};

    use super::*;

    #[test]
    fn concat3_size() {
        let value = crate::concat!('"', "hi", '"');

        assert_eq!(
            mem::size_of_val(&value),
            mem::size_of::<(char, &str, char)>()
        );
    }

    #[test]
    fn concat_tuple() {
        // Tests all tuple sizes through max.
        macro_rules! test {
            (@ $($all:ident),+) => {{
                let expected = core::concat!($(core::stringify!($all)),+);
                let value = crate::concat!($(core::stringify!($all)),+);

                assert_eq!(value.to_string(), expected);
            }};
            ($a:ident) => {
                test!(@ $a);
            };
            ($a:ident $(, $rest:ident)+) => {
                test!($($rest),+);
                test!(@ $a, $($rest),+);
            };
        }

        // `impl_tuple!` goes up to 12 tuple values, so this should test all
        // possible invocations.
        #[rustfmt::skip]
        test!(
            A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11,
            B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11
        );
    }

    /// Tests invoking `ConcatOnce` in a reference cycle.
    ///
    /// When run under Miri, this test ensures `Once` does not have UB.
    #[test]
    fn concat_once_cycle() {
        let rc: Rc<ConcatOnce<iter::FromFn<_>>> = Rc::new_cyclic(|rc| {
            let rc = Weak::clone(rc) as Weak<dyn Display>;

            let mut ran = false;
            let iter = iter::from_fn(move || {
                if ran {
                    return None;
                }
                ran = true;

                let rc = rc.upgrade().expect("`Rc` should be initialized");

                // Eagerly evaluate `rc`. Recursing should be a no-op.
                Some(alloc::format!("X{rc}"))
            });

            concat_once(iter)
        });

        assert_eq!(rc.to_string(), "X");
    }
}
