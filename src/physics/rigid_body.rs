use crate::math::Vector2D;
use crate::shape::Shape;
use crate::{Object, ObjectTrait};
use graphics::math::Scalar;
use graphics::*;
use opengl_graphics::*;

const G_CONST: f64 = 6.673e-11;

pub struct RigidBody {
    pub shape: Shape,
    pub center: Vector2D<Scalar>,
    /// SI Units
    pub mass: Scalar,
    pub velocity: Vector2D<Scalar>,
}

impl RigidBody {
    fn add_force(&mut self, force: &Vector2D<Scalar>) {
        self.velocity += *force / self.mass;
    }

    fn calc_gravitational_force(&self, body2: &mut RigidBody) -> Vector2D<Scalar> {
        let radius = body2.center - self.center;

        let radius_mag: Scalar = radius.clone().into();

        let force = (G_CONST * self.mass * body2.mass) / (radius_mag * radius_mag);

        Vector2D {
            x: force * radius.x,
            y: force * radius.y,
        }
    }

    fn calc_gravitational_attraction(&mut self, body2: &mut RigidBody) {
        let force = self.calc_gravitational_force(body2);
        self.add_force(&force);
        body2.add_force(&(force * -1.0));
    }
}

impl ObjectTrait for RigidBody {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        match &self.shape {
            Shape::RECTANGLE(r) => {
                g.tri_list(&c.draw_state, &r.color, |k| {
                    let x = self.center.x - (r.width / 2.);
                    let y = self.center.y - (r.height / 2.);
                    k(&triangulation::rect_tri_list_xy(
                        c.transform,
                        [x, y, r.width, r.height],
                    ));
                });
            }
        }
    }

    fn check(&mut self, obj: &mut Object) {
        match obj {
            Object::RigidBody(b) => {
                self.calc_gravitational_attraction(b);
            }
        }
    }

    fn calc_pos(&mut self, time: f64) {
        self.center += self.velocity * time;
    }
}

impl Into<Object> for RigidBody {
    fn into(self) -> Object {
        Object::RigidBody(self)
    }
}
