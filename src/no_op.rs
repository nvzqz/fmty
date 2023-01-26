pub(crate) mod types {
    #[allow(unused)]
    use super::*;

    /// See [`no_op()`].
    pub type NoOp = crate::types::ConcatTuple<()>;
}

use types::*;

/// Writes nothing.
///
/// # Examples
///
/// ```
/// assert_eq!(format!("{}", fmty::no_op()), "");
/// assert_eq!(format!("{:?}", fmty::no_op()), "");
/// ```
#[inline]
pub fn no_op() -> NoOp {
    crate::concat_tuple(())
}
