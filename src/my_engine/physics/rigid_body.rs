use crate::physics::*;
use crate::my_engine::math::Vector2D;
use graphics::*;
use opengl_graphics::*;
use graphics::math::Scalar;
use graphics::triangulation::{tx, ty};

pub struct Rectangle {
    pub pos: Vector2D<Scalar>,
    pub width: Scalar,
    pub height: Scalar,
    pub gravity: Scalar,
    pub color: [f32; 4],
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            pos: Vector2D {
                x: 0.,
                y: 0.,
            },
            width: 0.0,
            height: 0.0,
            gravity: 0.0,
            color: [0., 0., 0., 0.],
        }
    }
}


pub enum RigidShape {
    RECTANGLE(Rectangle)
}

pub struct RigidBody {
    pub shape: RigidShape
}

impl ObjectProps for RigidBody {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        match &self.shape {
            RigidShape::RECTANGLE(r) => {
                g.tri_list(&c.draw_state, &r.color, |k| {
                    k(&triangulation::rect_tri_list_xy
                        (c.transform,
                         [r.pos.x, r.pos.y, r.width, r.height])
                    );
                });
            }
        }
    }

    fn check_collisions(&mut self, obj: &mut Object) {
        match obj {
            Object::RigidBody(b) => {
                match &mut b.shape {
                    RECTANGLE => {}
                }
            }
        }
    }
}