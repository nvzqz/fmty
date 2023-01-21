use core::fmt::*;

pub(crate) mod types {
    #[allow(unused)]
    use super::*;

    /// See [`truncate_chars()`].
    #[derive(Clone, Copy)]
    pub struct TruncateChars<T> {
        pub(super) value: T,
        pub(super) len: usize,
    }
}

use types::*;

/// Shortens to `len` [`char`]s.
///
/// This may be used as a [`char`]-based alternative to
/// [`str[..len]`](https://doc.rust-lang.org/std/primitive.str.html#method.index)
/// or [`String::truncate()`](https://doc.rust-lang.org/std/string/struct.String.html#method.truncate).
///
/// # Examples
///
/// ```
/// let value = fmty::truncate_chars(123, 2);
/// assert_eq!(value.to_string(), "12");
/// ```
pub fn truncate_chars<T>(value: T, len: usize) -> TruncateChars<T> {
    TruncateChars { value, len }
}

impl<T: Display> Display for TruncateChars<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        struct Writer<'a, 'b> {
            f: &'a mut Formatter<'b>,
            rem_len: usize,
        }

        impl Write for Writer<'_, '_> {
            fn write_str(&mut self, mut s: &str) -> Result {
                if self.rem_len == 0 {
                    return Ok(());
                }

                // We want to `.take()` 1 past `rem_len` so that we get the byte
                // index of where the last target `char` ends.
                let take_len = match self.rem_len.checked_add(1) {
                    Some(n) => n,
                    None => return self.f.write_str(s),
                };

                if let Some((char_offset, (byte_offset, _))) =
                    s.char_indices().enumerate().take(take_len).last()
                {
                    if char_offset == self.rem_len {
                        s = &s[..byte_offset];
                        self.rem_len = 0;
                    } else {
                        self.rem_len -= char_offset + 1;
                    }
                } else {
                    // Empty iterator.
                    return Ok(());
                }

                self.f.write_str(s)
            }

            #[inline]
            fn write_char(&mut self, c: char) -> Result {
                if let Some(rem_len) = self.rem_len.checked_sub(1) {
                    self.rem_len = rem_len;
                    self.f.write_char(c)
                } else {
                    Ok(())
                }
            }

            #[inline]
            fn write_fmt(&mut self, args: Arguments) -> Result {
                if self.rem_len == 0 {
                    Ok(())
                } else {
                    write(self, args)
                }
            }
        }

        write!(Writer { f, rem_len: self.len }, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn chars() {
        let expected = &"abc";

        for len in 0..=expected.len() {
            let expected = &expected[..len];

            assert_eq!(
                truncate_chars(format_args!("{}{}{}", 'a', 'b', 'c'), len)
                    .to_string(),
                expected,
                "incorrect result for length {len}",
            )
        }
    }

    #[test]
    fn two_parts() {
        let expected = &"abc123";

        for len in 0..=expected.len() {
            let expected = &expected[..len];

            assert_eq!(
                truncate_chars(format_args!("abc{}", 123), len).to_string(),
                expected,
                "incorrect result for length {len}",
            );
        }
    }

    #[test]
    fn three_parts() {
        let expected = &"abc123xyz";

        for len in 0..=expected.len() {
            let expected = &expected[..len];

            assert_eq!(
                truncate_chars(format_args!("abc{}xyz", 123), len).to_string(),
                expected,
                "incorrect result for length {len}",
            );
        }
    }
}
