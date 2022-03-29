use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub};

use graphics::math::Scalar;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Vector2D<T> {
    type Output = Vector2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector2D<T> {
    type Output = Vector2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Index<i8> for Vector2D<T> {
    type Output = T;

    fn index(&self, index: i8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vector2D index out of bounds"),
        }
    }
}

/// dot product
impl Mul<Vector2D<Scalar>> for Vector2D<Scalar> {
    type Output = Scalar;

    fn mul(self, rhs: Vector2D<Scalar>) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

/// multiplication with scalar
impl Mul<Scalar> for Vector2D<Scalar> {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self {
            x: (self.x * rhs),
            y: (self.y * rhs),
        }
    }
}

/// divide with scalar
impl Div<Scalar> for Vector2D<Scalar> {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        Self {
            x: (self.x / rhs),
            y: (self.y / rhs),
        }
    }
}

impl Into<Scalar> for Vector2D<Scalar> {
    fn into(self) -> Scalar {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

impl<T> IndexMut<i8> for Vector2D<T> {
    fn index_mut(&mut self, index: i8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vector2D Mutable Borrow, index out of bounds"),
        }
    }
}
