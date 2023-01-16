use core::{fmt::*, iter};

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

/// Implements [`Display`] by concatenating [`Iterator::map()`] results.
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
    F: Fn(I::Item) -> R + Clone,
{
    concat(iter.into_iter().map(f))
}

/// Implements [`Display`] by concatenating [tuple](prim@tuple) items that may
/// be different types.
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

/// See [`concat()`].
#[derive(Clone, Copy)]
pub struct Concat<I> {
    iter: I,
}

/// See [`concat_map()`].
pub type ConcatMap<I, F> = Concat<iter::Map<I, F>>;

/// See [`concat_tuple()`].
#[derive(Clone, Copy)]
pub struct ConcatTuple<T>(T);

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

impl Display for ConcatTuple<()> {
    #[inline]
    fn fmt(&self, _: &mut Formatter) -> Result {
        Ok(())
    }
}

/// Implements `Display` for `ConcatTuple<(T, ...)>`.
macro_rules! impl_tuple {
    () => {};
    ($($x:ident),+) => {
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

        impl_peel!($($x),+);
    };
}

macro_rules! impl_peel {
    ($x:ident $(, $rest:ident)*) => {
        impl_tuple!($($rest),*);
    };
}

impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);

/// Implements [`Display`] by concatenating items that may be different types.
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
    use alloc::string::ToString;
    use core::mem;

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
}
