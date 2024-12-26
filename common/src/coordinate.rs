use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coordinate(pub usize, pub usize);

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coordinate(self.0 + other.0, self.1 + other.1)
    }
}
