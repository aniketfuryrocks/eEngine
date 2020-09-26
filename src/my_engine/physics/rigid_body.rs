use crate::physics::*;
use crate::my_engine::math::Vector2D;
use graphics::*;
use opengl_graphics::*;
use graphics::math::Scalar;
use graphics::triangulation::{tx, ty};

pub struct Rectangle {
    pub pos:Vector2D<f32>,
    pub width:f32,
    pub height:f32,
    pub color:[f32;4]
}

pub enum RigidShape {
    RECTANGLE(Rectangle)
}

pub struct RigidBody {
    pub shape:RigidShape
}

impl ObjectProps for RigidBody {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        match &self.shape {
            RigidShape::RECTANGLE(r) => {
                g.tri_list(&c.draw_state,&r.color,|k|{
                    k(&triangulation::rect_tri_list_xy
                        (c.transform,
                        [r.pos.x as f64, r.pos.y as f64, r.width as f64, r.height as f64])
                    );
                    //implementation of above
                    /*let (x, y, w, h) = (r.pos.x as f64 , r.pos.y as f64 , r.width as f64, r.height as f64);
                    let (x2, y2) = (x + w, y + h);
                    let m = c.transform;
                    k(&[[tx(m, x, y), ty(m, x, y)],
                        [tx(m, x2, y), ty(m, x2, y)],
                        [tx(m, x, y2), ty(m, x, y2)],
                        [tx(m, x2, y), ty(m, x2, y)],
                        [tx(m, x2, y2), ty(m, x2, y2)],
                        [tx(m, x, y2), ty(m, x, y2)]])*/
                    /*
                    let (x, y, w, h) = (r.pos.x  , r.pos.y  , r.width , r.height );
                    k(&[[x,y],[x+w,y+h],[x+123.,y+123.],
                        [x,y],[x+w,y+h],[x+123.,y+123.]])*/
                });
            }
        }
    }

    fn check_collisions(&mut self, obj: &mut Object) {
        match obj {
            Object::RigidBody(b)=> {
                match &mut b.shape {
                    RECTANGLE=> {

                    }
                }
            }
        }
    }

}