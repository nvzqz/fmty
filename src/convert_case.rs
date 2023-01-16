use core::fmt::*;

/// Converts the [`Display`] formatting to uppercase.
///
/// This may be used as a non-allocating alternative to
/// [`str::to_uppercase()`](https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase).
pub fn to_uppercase<T>(value: T) -> ToUppercase<T> {
    ToUppercase { value }
}

/// Converts the [`Display`] formatting to lowercase.
///
/// This may be used as a non-allocating alternative to
/// [`str::to_lowercase()`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase).
pub fn to_lowercase<T>(value: T) -> ToLowercase<T> {
    ToLowercase { value }
}

/// See [`to_uppercase()`].
#[derive(Clone, Copy)]
pub struct ToUppercase<T> {
    value: T,
}

/// See [`to_lowercase()`].
#[derive(Clone, Copy)]
pub struct ToLowercase<T> {
    value: T,
}

impl<T: Display> Display for ToUppercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        struct Writer<'a, 'b> {
            f: &'a mut Formatter<'b>,
        }

        impl Write for Writer<'_, '_> {
            fn write_str(&mut self, s: &str) -> Result {
                for c in s.chars() {
                    self.write_char(c)?;
                }
                Ok(())
            }

            fn write_char(&mut self, c: char) -> Result {
                for c in c.to_uppercase() {
                    self.f.write_char(c)?;
                }
                Ok(())
            }
        }

        write!(Writer { f }, "{}", self.value)
    }
}

impl<T: Display> Display for ToLowercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        struct Writer<'a, 'b> {
            f: &'a mut Formatter<'b>,
        }

        impl Write for Writer<'_, '_> {
            fn write_str(&mut self, s: &str) -> Result {
                for c in s.chars() {
                    self.write_char(c)?;
                }
                Ok(())
            }

            fn write_char(&mut self, c: char) -> Result {
                for c in c.to_lowercase() {
                    self.f.write_char(c)?;
                }
                Ok(())
            }
        }

        write!(Writer { f }, "{}", self.value)
    }
}
