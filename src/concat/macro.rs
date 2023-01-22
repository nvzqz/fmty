/// Concatenates items that may be different types.
///
/// This is like [`concat_tuple()`](crate::concat_tuple) but with no length
/// limit.
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
    () => { $crate::concat_tuple(()) };
    ($t0:expr $(,)?) => { $crate::concat_tuple(($t0,)) };
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
    use std::mem;

    #[test]
    fn concat3_size() {
        let value = crate::concat!('"', "hi", '"');

        assert_eq!(
            mem::size_of_val(&value),
            mem::size_of::<(char, &str, char)>()
        );
    }
}
