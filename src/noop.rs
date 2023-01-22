pub(crate) mod types {
    #[allow(unused)]
    use super::*;

    /// See [`noop()`].
    pub type NoOp = crate::types::ConcatTuple<()>;
}

use types::*;

/// Writes nothing (no-op).
///
/// # Examples
///
/// ```
/// assert_eq!(format!("{}", fmty::noop()), "");
/// assert_eq!(format!("{:?}", fmty::noop()), "");
/// ```
#[inline]
pub fn noop() -> NoOp {
    crate::concat_tuple(())
}
