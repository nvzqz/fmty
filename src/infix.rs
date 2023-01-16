use core::fmt::*;

/// Implements [`Display`] by placing a value between `left` and `right`.
pub fn infix<T, L, R>(left: L, value: T, right: R) -> Infix<T, L, R> {
    Infix { value, left, right }
}

/// See [`infix()`].
#[derive(Clone, Copy)]
pub struct Infix<T, L = T, R = L> {
    value: T,
    left: L,
    right: R,
}

impl<T, L, R> Display for Infix<T, L, R>
where
    T: Display,
    L: Display,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}{}", self.left, self.value, self.right)
    }
}
