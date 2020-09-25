use std::ops::Index;

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