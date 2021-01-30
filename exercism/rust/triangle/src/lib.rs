use std::cmp::{PartialEq,PartialOrd};
use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3]
}

impl<T> Triangle<T>
    where
        T: Clone + Copy + Add<Output = T> + Default + PartialEq + PartialOrd
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.len() != 3 || sides.iter().any(|x| *x == T::default()) {
            return None;
        }

        if (sides[0] + sides[1] < sides[2])
            || (sides[1] + sides[2] < sides[0])
            || (sides[2] + sides[0] < sides[1]) {
            return None;
        }

        Some(Triangle { sides: sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().all(|x| *x == self.sides[0])
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2] && self.sides[0] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2] || self.sides[0] == self.sides[2]
    }
}
