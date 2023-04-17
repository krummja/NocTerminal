use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

use num::{
    self,
    integer::Integer,
};
use num_traits::{
    float::Float,
    Signed,
};

use crate::point::Point;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T: Sub<Output = T> + Signed> Size<T> {
    pub fn from_points(first: Point<T>, second: Point<T>) -> Self {
        let width = num::abs(first.x - second.x);
        let height = num::abs(first.y - second.y);
        Self { width, height }
    }
}

impl<T: Float> Size<T> {
    pub fn floor(&self) -> Self where T: Float {
        Self {
            width: self.width.floor(),
            height: self.height.floor(),
        }
    }
}

impl<T: Integer> Size<T> {
    pub fn area(&self) -> T where T: Mul<Output = T> + Copy {
        self.width * self.height
    }
}

impl<T: Add<Output = T>> Add for Size<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Size<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Size<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl<T: Div<Output = T> + PartialOrd<i32>> Div for Size<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if other.width == 0 || other.height == 0 {
            panic!("Cannot divide by zero-valued `Size`");
        }

        Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::size::Size;
    use crate::point::Point;

    #[test]
    fn test_area() {
        let size = Size { width: 10, height: 10 };
        assert_eq!(size.area(), 100);
    }

    #[test]
    fn test_floor() {
        let size = Size { width: 10.5, height: 11.23 };
        assert_eq!(size.floor(), Size { width: 10.0, height: 11.0 });
    }

    #[test]
    fn test_float_addition() {
        let s1 = Size { width: 10.5, height: 10.5 };
        let s2 = Size { width: 10.5, height: 10.5 };
        assert_eq!(s1 + s2, Size { width: 21.0, height: 21.0 });
    }

    #[test]
    fn test_integer_addition() {
        let s1 = Size { width: 10, height: 10 };
        let s2 = Size { width: 10, height: 10 };
        assert_eq!(s1 + s2, Size { width: 20, height: 20 });
    }

    #[test]
    fn test_from_points() {
        let size = Size::from_points(
            Point { x: 10, y: 20 },
            Point { x: 30, y: -2 },
        );

        assert_eq!(size, Size { width: 20, height: 22 });
    }
}
