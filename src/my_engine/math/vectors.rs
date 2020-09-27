use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Vector2D<T> {
    pub x:T,
    pub y:T
}

impl<T> Index<i8> for Vector2D<T> {
    type Output = T;

    fn index(&self, index: i8) -> &Self::Output {
        match index {
            0=> &self.x,
            1=> &self.y,
            _=> panic!("Vector2D index out of bounds")
        }
    }
}

impl<T> IndexMut<i8> for Vector2D<T> {
    fn index_mut(&mut self, index: i8) -> &mut Self::Output {
        match index {
            0=> &mut self.x,
            1=> &mut self.y,
            _=> panic!("Vector2D Mutable Borrow, index out of bounds")
        }
    }
}