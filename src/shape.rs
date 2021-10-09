use graphics::math::Scalar;

#[derive(Default)]
pub struct Rectangle {
    pub width: Scalar,  //m
    pub height: Scalar, //m
    pub color: [f32; 4],
}

pub enum Shape {
    RECTANGLE(self::Rectangle),
}

impl From<Rectangle> for Shape {
    fn from(r: Rectangle) -> Self {
        Self::RECTANGLE(r)
    }
}
