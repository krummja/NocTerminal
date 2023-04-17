use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

use num::integer::Integer;
use num_traits::{
    float::Float,
    Signed,
};

use crate::point::Point;
use crate::size::Size;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_point(point: Point<T>, width: T, height: T) -> Self {
        Self {
            x: point.x,
            y: point.y,
            width, height,
        }
    }

    pub fn from_size(x: T, y: T, size: Size<T>) -> Self {
        Self {
            x, y,
            width: size.width,
            height: size.height,
        }
    }

    pub fn from_point_size(point: Point<T>, size: Size<T>) -> Self {
        Self {
            x: point.x,
            y: point.y,
            width: size.width,
            height: size.height,
        }
    }

    pub fn size(&self) -> Size<T> where T: Copy {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}

impl<T: Signed> Rectangle<T> {
    pub fn area(&self) -> T where T: Mul<Output = T> + Copy {
        self.width * self.height
    }
}

impl<T: Float> Rectangle<T> {
    pub fn floor(&self) -> Self where T: Float {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            width: self.width.floor(),
            height: self.height.floor(),
        }
    }
}

impl<T: Add<Output = T>> Add for Rectangle<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Rectangle<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Rectangle<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl<T: Div<Output = T> + PartialOrd<i32>> Div for Rectangle<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if (
            other.x == 0 ||
            other.y == 0 ||
            other.width == 0 ||
            other.height == 0
        ) {
            panic!("Cannot divide by zero-valued `Rectangle`");
        }

        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}


#[test]
fn test_add() {
    let r1 = Rectangle { x: 1, y: 1, width: 3, height: 10 };
    let r2 = Rectangle { x: 1, y: 1, width: 3, height: 10 };

    assert_eq!(r1 + r2, Rectangle { x: 2, y: 2, width: 6, height: 20 });
}

#[test]
fn test_area() {
    let r1 = Rectangle { x: 0, y: 0, width: 10, height: 10 };
    let r2 = Rectangle { x: 0.0, y: 0.0, width: 1.3, height: 3.4 };
    assert_eq!(r1.area(), 100);
    assert_eq!(r2.area(), 4.42);
}
