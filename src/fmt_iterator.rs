use crate::{types::*, *};

/// [`Iterator`] formatting methods.
pub trait FmtIterator: Iterator + Sized {
    /// Method for [`concat()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_concat();
    /// assert_eq!(value.to_string(), "holamundo");
    /// ```
    fn fmt_concat(self) -> Concat<Self>
    where
        Self: Clone,
    {
        concat(self)
    }

    /// Method for [`concat_once()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_concat_once();
    /// assert_eq!(value.to_string(), "holamundo");
    ///
    /// assert_eq!(value.to_string(), "");
    /// ```
    fn fmt_concat_once(self) -> ConcatOnce<Self> {
        concat_once(self)
    }

    /// Method for [`concat_map()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_concat_map(fmty::to_uppercase);
    /// assert_eq!(value.to_string(), "HOLAMUNDO");
    /// ```
    fn fmt_concat_map<R, F>(self, f: F) -> ConcatMap<Self, F>
    where
        Self: Clone,
        F: Fn(Self::Item) -> R,
    {
        concat_map(self, f)
    }

    /// Method for [`concat_map_once()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_concat_map_once(fmty::to_uppercase);
    /// assert_eq!(value.to_string(), "HOLAMUNDO");
    ///
    /// assert_eq!(value.to_string(), "");
    /// ```
    fn fmt_concat_map_once<R, F>(self, f: F) -> ConcatMapOnce<Self, F>
    where
        F: Fn(Self::Item) -> R,
    {
        concat_map_once(self, f)
    }

    /// Method for [`join()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_join(" ");
    /// assert_eq!(value.to_string(), "hola mundo");
    /// ```
    fn fmt_join<S>(self, sep: S) -> Join<Self, S>
    where
        Self: Clone,
    {
        join(self, sep)
    }

    /// Method for [`join_once()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_join_once(" ");
    /// assert_eq!(value.to_string(), "hola mundo");
    ///
    /// assert_eq!(value.to_string(), "");
    /// ```
    fn fmt_join_once<S>(self, sep: S) -> JoinOnce<Self, S> {
        join_once(self, sep)
    }

    /// Method for [`join_map()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_join_map(" ", fmty::to_uppercase);
    /// assert_eq!(value.to_string(), "HOLA MUNDO");
    /// ```
    fn fmt_join_map<S, R, F>(self, sep: S, f: F) -> JoinMap<Self, S, F>
    where
        Self: Clone,
        F: Fn(Self::Item) -> R,
    {
        join_map(self, sep, f)
    }

    /// Method for [`join_map_once()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_join_map_once(" ", fmty::to_uppercase);
    /// assert_eq!(value.to_string(), "HOLA MUNDO");
    ///
    /// assert_eq!(value.to_string(), "");
    /// ```
    fn fmt_join_map_once<S, R, F>(self, sep: S, f: F) -> JoinMapOnce<Self, S, F>
    where
        F: Fn(Self::Item) -> R,
    {
        join_map_once(self, sep, f)
    }

    /// Method for [`csv()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_csv();
    /// assert_eq!(value.to_string(), "hola, mundo");
    /// ```
    fn fmt_csv(self) -> Csv<Self>
    where
        Self: Clone,
    {
        csv(self)
    }

    /// Method for [`csv_once()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_csv_once();
    /// assert_eq!(value.to_string(), "hola, mundo");
    ///
    /// assert_eq!(value.to_string(), "");
    /// ```
    fn fmt_csv_once(self) -> CsvOnce<Self> {
        csv_once(self)
    }

    /// Method for [`csv_map()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_csv_map(fmty::to_uppercase);
    /// assert_eq!(value.to_string(), "HOLA, MUNDO");
    /// ```
    fn fmt_csv_map<R, F>(self, f: F) -> CsvMap<Self, F>
    where
        Self: Clone,
        F: Fn(Self::Item) -> R,
    {
        csv_map(self, f)
    }

    /// Method for [`csv_map_once()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fmty::FmtIterator;
    ///
    /// let value = ["hola", "mundo"].iter().fmt_csv_map_once(fmty::to_uppercase);
    /// assert_eq!(value.to_string(), "HOLA, MUNDO");
    ///
    /// assert_eq!(value.to_string(), "");
    /// ```
    fn fmt_csv_map_once<R, F>(self, f: F) -> CsvMapOnce<Self, F>
    where
        F: Fn(Self::Item) -> R,
    {
        csv_map_once(self, f)
    }
}

impl<I: Iterator> FmtIterator for I {}
