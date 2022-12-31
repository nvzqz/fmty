use core::fmt::*;

#[derive(Clone, Copy)]
pub struct Concat<T>(pub T);

impl<A, B> Display for Concat<(A, B)>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let (a, b) = &self.0;
        write!(f, "{a}{b}")
    }
}

impl<A, B, C> Display for Concat<(A, B, C)>
where
    A: Display,
    B: Display,
    C: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let (a, b, c) = &self.0;
        write!(f, "{a}{b}{c}")
    }
}
