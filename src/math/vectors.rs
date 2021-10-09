use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

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

impl<T: Mul<Output = T> + Clone> Mul<T> for Vector2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
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
