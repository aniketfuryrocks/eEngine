use crate::my_engine::physics::RigidBody;
use graphics::Context;
use opengl_graphics::GlGraphics;

pub enum Object {
    RigidBody(RigidBody),
}

pub trait ObjectProps {
    fn draw(&self, c: &Context, g: &mut GlGraphics);
    fn check_collisions(&mut self, obj: &mut Object);
}

impl ObjectProps for Object {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        match self {
            Object::RigidBody(r) => r.draw(c, g)
        }
    }

    fn check_collisions(&mut self, obj: &mut Object) {
        match self {
            Object::RigidBody(r) => r.check_collisions(obj)
        }
    }
}