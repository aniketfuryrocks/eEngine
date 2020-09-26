use crate::physics::*;
use crate::my_engine::math::Vector2D;
use graphics::*;
use opengl_graphics::*;
use graphics::math::Scalar;
use graphics::triangulation::{tx, ty};

//time for 1 frame in 60 fps for 1 second windows
const TIME:f64 = 1.0/25.0;
const G_CONST:f64 = 6.673e-11;

pub struct Rectangle {
    pub width: Scalar,//m
    pub height: Scalar,//m
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
    pub mass:Scalar,//kg
    pub velocity:Scalar,//m/s
}

impl RigidBody {
    /// Calculate attraction between two bodies using newtons law
    /// Formulae ->
    /// For Body m1 final v = (G * m2 * T / r) + u;
    fn calc_attr(&mut self,body: &mut RigidBody){
        //calculate angle (radians) and distance between the 2
        let (sin_theta,cos_theta,r) =  {
            let y_diff = body.center.y - self.center.y;
            let x_diff = body.center.x - self.center.x;
            let distance:f64 = (y_diff*y_diff * x_diff * x_diff).sqrt();
            let theta = (y_diff/x_diff).atan();
            (theta.sin(),theta.cos(),distance)
        };

        //calculate final velocity
        self.velocity = (G_CONST * body.mass * TIME / r) + self.velocity;
        body.velocity = (G_CONST * self.mass * TIME / r) + body.velocity;

        //Sum of components
        {
            //calculate displacement from v*t=S
            let s = self.velocity * TIME;
            //using Ax = A cos (theta)
            let ax = s * cos_theta;//x component
            let ay = s * sin_theta;//y component
            //resultant
            self.center.x = ax + self.center.x;
            self.center.y = ay + self.center.y;
        }

        {
            //calculate displacement from v*t=S
            let s = body.velocity * TIME;
            //using Ax = A cos (theta)
            let ax = s * cos_theta;//x component
            let ay = s * sin_theta;//y component
            //resultant
            body.center.x = ax + body.center.x;
            body.center.y = ay + body.center.y;
        }
    }
}




impl ObjectProps for RigidBody {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        match &self.shape {
            RigidShape::RECTANGLE(r) => {
                g.tri_list(&c.draw_state, &r.color, |k| {
                    let x = self.center.x - (r.width/2.);
                    let y = self.center.y + (r.height/2.);
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
    } }