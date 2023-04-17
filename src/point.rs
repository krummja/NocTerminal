use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

use num::integer::Integer;
use num_traits::float::Float;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Float> Point<T> {
    pub fn floor(&self) -> Self where T: Float {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Point<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: Div<Output = T> + PartialOrd<i32>> Div for Point<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if other.x == 0 || other.y == 0 {
            panic!("Cannot divide by zero-valued `Point`");
        }

        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::point::Point;

    #[test]
    fn test_addition() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 25, y: -20 };

        let result = p1 + p2;

        assert!(result.x == 10 + 25);
        assert!(result.y == 10 - 20);
    }

    #[test]
    fn test_subtraction() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 25, y: -20 };

        let result = p1 - p2;

        assert!(result.x == 10 - 25);
        assert!(result.y == 10 + 20);
    }

    #[test]
    fn test_multiplication() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 25, y: -20 };

        let result = p1 * p2;

        assert!(result.x == 10 * 25);
        assert!(result.y == 10 * -20);
    }

    #[test]
    fn test_division() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 25, y: -20 };

        let result = p1 / p2;

        assert!(result.x == 10 / 25);
        assert!(result.y == 10 / -20);
    }

    #[test]
    #[should_panic]
    fn test_div_zero_panic() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 0, y: 20 };

        let result = p1 / p2;
    }

    #[test]
    fn test_equality() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 10, y: 10 };
        let p3 = Point { x: 20, y: 10 };

        let result1 = p1 == p2;
        let result2 = p2 == p3;

        assert!(result1 == true);
        assert!(result2 == false);
    }
}
