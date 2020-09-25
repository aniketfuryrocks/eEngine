use crate::physics::*;
use graphics::math::{Scalar};
use crate::my_engine::math::Vector2D;

pub enum RigidShape {
    RECTANGLE,
    ELLIPSE
}

pub struct RigidBody {
    pub pos:Vector2D<Scalar>,
    pub shape:RigidShape
}

impl Object for RigidBody {
    fn draw(&self) {
        self.pos[0];
    }
}