pub(crate) mod types {
    #[allow(unused)]
    use super::*;

    /// See [`infix()`].
    pub type Infix<T, L = T, R = L> = crate::types::ConcatTuple<(L, T, R)>;
}

use types::*;

/// Places a value between `left` and `right`.
pub fn infix<T, L, R>(left: L, value: T, right: R) -> Infix<T, L, R> {
    crate::concat_tuple((left, value, right))
}
