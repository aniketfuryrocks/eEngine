use graphics::Context;
use opengl_graphics::GlGraphics;

use crate::physics::RigidBody;

pub enum Object {
    RigidBody(RigidBody),
}

pub trait ObjectTrait: Into<Object> {
    fn draw(&self, c: &Context, g: &mut GlGraphics);
    fn check(&mut self, obj: &mut Object);
    fn calc_pos(&mut self, time: f64);
}

impl ObjectTrait for Object {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        match self {
            Object::RigidBody(r) => r.draw(c, g),
        }
    }

    fn check(&mut self, obj: &mut Object) {
        match self {
            Object::RigidBody(r) => r.check(obj),
        }
    }

    fn calc_pos(&mut self, time: f64) {
        match self {
            Object::RigidBody(r) => r.calc_pos(time),
        }
    }
}
