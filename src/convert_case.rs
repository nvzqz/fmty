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

/// Converts the [`Display`] formatting to ASCII uppercase.
///
/// This may be used as a non-allocating alternative to
/// [`str::to_ascii_uppercase()`](https://doc.rust-lang.org/std/primitive.str.html#method.to_ascii_uppercase).
///
/// # Examples
///
/// ```
/// let value = fmty::to_ascii_uppercase("Grüße, Jürgen ❤");
/// assert_eq!(value.to_string(), "GRüßE, JüRGEN ❤");
/// ```
pub fn to_ascii_uppercase<T>(value: T) -> ToAsciiUppercase<T> {
    ToAsciiUppercase { value }
}

/// Converts the [`Display`] formatting to ASCII lowercase.
///
/// This may be used as a non-allocating alternative to
/// [`str::to_ascii_lowercase()`](https://doc.rust-lang.org/std/primitive.str.html#method.to_ascii_lowercase).
///
/// # Examples
///
/// ```
/// let value = fmty::to_ascii_lowercase("Grüße, Jürgen ❤");
/// assert_eq!(value.to_string(), "grüße, jürgen ❤");
/// ```
pub fn to_ascii_lowercase<T>(value: T) -> ToAsciiLowercase<T> {
    ToAsciiLowercase { value }
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

/// See [`to_ascii_uppercase()`].
#[derive(Clone, Copy)]
pub struct ToAsciiUppercase<T> {
    value: T,
}

/// See [`to_ascii_lowercase()`].
#[derive(Clone, Copy)]
pub struct ToAsciiLowercase<T> {
    value: T,
}

/// Single writer for ASCII to reduce code generation.
struct AsciiWriter<'a, 'b> {
    f: &'b mut Formatter<'a>,
    uppercase: bool,
}

impl Write for AsciiWriter<'_, '_> {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result {
        self.f.write_char(if self.uppercase {
            c.to_ascii_uppercase()
        } else {
            c.to_ascii_lowercase()
        })
    }
}

struct UppercaseWriter<'a, 'b> {
    f: &'a mut Formatter<'b>,
}

impl Write for UppercaseWriter<'_, '_> {
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

struct LowercaseWriter<'a, 'b> {
    f: &'a mut Formatter<'b>,
}

impl Write for LowercaseWriter<'_, '_> {
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

impl<T: Debug> Debug for ToUppercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(UppercaseWriter { f }, "{:?}", self.value)
    }
}

impl<T: Display> Display for ToUppercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(UppercaseWriter { f }, "{}", self.value)
    }
}

impl<T: Debug> Debug for ToLowercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(LowercaseWriter { f }, "{:?}", self.value)
    }
}

impl<T: Display> Display for ToLowercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(LowercaseWriter { f }, "{}", self.value)
    }
}

impl<T: Debug> Debug for ToAsciiLowercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(AsciiWriter { f, uppercase: false }, "{:?}", self.value)
    }
}

impl<T: Display> Display for ToAsciiLowercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(AsciiWriter { f, uppercase: false }, "{}", self.value)
    }
}

impl<T: Debug> Debug for ToAsciiUppercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(AsciiWriter { f, uppercase: true }, "{:?}", self.value)
    }
}

impl<T: Display> Display for ToAsciiUppercase<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(AsciiWriter { f, uppercase: true }, "{}", self.value)
    }
}
