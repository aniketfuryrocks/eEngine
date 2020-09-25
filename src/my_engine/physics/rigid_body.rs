use crate::physics::*;
use graphics::math::{Scalar};
use crate::my_engine::math::Vector2D;

#[derive(Debug)]
pub enum RigidShape {
    RECTANGLE,
    ELLIPSE
}

pub struct RigidBody {
    pub pos:Vector2D<Scalar>,
    pub shape:RigidShape
}

impl ObjectProps for RigidBody {
    fn draw(&self) {
        self.pos[0];
    }

    fn collides(&self,object: &Object) {
      //  let body = object as RigidBody;
        //object.
    }
}