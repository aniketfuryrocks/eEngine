use crate::physics::*;
use crate::my_engine::math::Vector2D;
use graphics::*;
use opengl_graphics::*;
use graphics::math::Scalar;
use graphics::triangulation::{tx, ty};

//time for 1 frame in 60 fps for 1 second windows
const TIME: f64 = 1.0;
const G_CONST: f64 = 6.673e-11;

pub struct Rectangle {
    pub width: Scalar,
    //m
    pub height: Scalar,
    //m
    pub color: [f32; 4],
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            width: 0.0,
            height: 0.0,
            color: [0., 0., 0., 0.],
        }
    }
}


pub enum RigidShape {
    RECTANGLE(Rectangle)
}

pub struct RigidBody {
    pub shape: RigidShape,
    pub center: Vector2D<Scalar>,
    pub mass: Scalar,
    //kg
    pub velocity: Scalar,//m/s
}

impl RigidBody {
    /// Calculate attraction between two bodies using newtons law
    /// Formulae ->
    /// For Body m1 final v = (G * m2 * T / r) + u;
    fn calc_attr(&mut self, body2: &mut RigidBody) {
        //need to sort first
        let (body1, body2) = {
            if body2.center.x < self.center.x || body2.center.y < body2.center.y {
                (body2, self)
            } else {
                (self, body2)
            }
        };
        //calculate angle (radians) and distance between the 2
        let (sin_theta, cos_theta, r) = {
            let y_diff = body2.center.y - body1.center.y;
            let x_diff = body2.center.x - body1.center.x;
            let distance: f64 = (y_diff * y_diff * x_diff * x_diff).sqrt();
            let theta = (y_diff / x_diff).atan();
            (theta.sin(), theta.cos(), distance)
        };

        //calculate final velocity
        body1.velocity = (G_CONST * body2.mass * TIME / r) + body1.velocity;
        body2.velocity = (G_CONST * body1.mass * TIME / r) + body2.velocity;

        //Sum of components
        {
            //calculate displacement from v*t=S
            let s = body1.velocity * TIME;
            //using Ax = A cos (theta)
            let ax = s * cos_theta;//x component
            let ay = s * sin_theta;//y component
            //resultant
            body1.center.x = ax + body1.center.x;
            body1.center.y = ay + body1.center.y;
        }

        {
            //calculate displacement from v*t=S
            let s = body2.velocity * TIME;
            //using Ax = A cos (theta)
            let ax = s * cos_theta;//x component
            let ay = s * sin_theta;//y component
            //resultant
            body2.center.x = body2.center.x - ax;
            body2.center.y = body2.center.y - ay;
        }
    }
}


impl ObjectProps for RigidBody {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        match &self.shape {
            RigidShape::RECTANGLE(r) => {
                g.tri_list(&c.draw_state, &r.color, |k| {
                    let x = self.center.x - (r.width / 2.);
                    let y = self.center.y + (r.height / 2.);
                    k(&triangulation::rect_tri_list_xy
                        (c.transform,
                         [x, y, r.width, r.height])
                    );
                });
            }
        }
    }

    fn check(&mut self, obj: &mut Object) {
        match obj {
            Object::RigidBody(b) => {
                self.calc_attr(b);
            }
        }
    }
}