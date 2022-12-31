use core::fmt::*;

#[derive(Clone, Copy)]
pub struct Concat<T>(pub T);

impl<T: Display, U: Display> Display for Concat<(T, U)> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let (x, y) = &self.0;
        write!(f, "{x}{y}")
    }
}
