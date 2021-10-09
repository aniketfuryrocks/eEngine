use crate::math::Vector2D;
use crate::shape::Shape;
use crate::{Object, ObjectTrait};
use graphics::math::Scalar;
use graphics::*;
use opengl_graphics::*;

//time for 1 frame in 60 fps for 1 second windows
const G_CONST: f64 = 6.673e-11;

pub struct RigidBody {
    pub shape: Shape,
    pub center: Vector2D<Scalar>,
    /// SI Units
    pub mass: Scalar,
    pub velocity: Vector2D<Scalar>,
}

impl RigidBody {
    /// Calculate attraction between two bodies using newtons law
    /// Formulae ->
    /// For Body m1 final v = (G * m2 * T / r^2) + u;
    fn calc_gravitational_attraction(&mut self, body2: &mut RigidBody, time: f64) {
        let body1 = self;
        let time = 86400.0e3 * time;
        //calculate angle (radians) and distance between the 2
        let (sin_theta, cos_theta, r) = {
            let perpendicular = body2.center.y - body1.center.y;
            let base = body2.center.x - body1.center.x;
            let r: f64 = ((perpendicular * perpendicular) + (base * base)).sqrt();
            // sin q = perpendicular / hypotenuse
            // cos q = base / hypotenuse
            (perpendicular / r, base / r, r)
        };

        {
            let gtr_pro = (G_CONST * time) / (r * r);
            //calculate final velocity
            let vel = body2.mass * gtr_pro;
            let vel = Vector2D {
                x: vel * cos_theta,
                y: vel * sin_theta,
            };

            body1.velocity += vel;

            let vel = body1.mass * gtr_pro;
            let vel = Vector2D {
                x: vel * cos_theta,
                y: vel * sin_theta,
            };

            body2.velocity += vel * -1.0f64;
        }
    }
}

impl ObjectTrait for RigidBody {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        match &self.shape {
            Shape::RECTANGLE(r) => {
                g.tri_list(&c.draw_state, &r.color, |k| {
                    let x = self.center.x - (r.width / 2.);
                    let y = self.center.y + (r.height / 2.);
                    k(&triangulation::rect_tri_list_xy(
                        c.transform,
                        [x, y, r.width, r.height],
                    ));
                });
            }
        }
    }

    fn check(&mut self, obj: &mut Object, time: f64) {
        match obj {
            Object::RigidBody(b) => {
                self.calc_gravitational_attraction(b, time);
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
