use crate::my_engine::physics::RigidBody;
use crate::my_engine::physics::Object::RIGID_BODY;

pub enum Object {
    RIGID_BODY(RigidBody)
}

pub trait ObjectProps {
    fn draw(&self);
    fn collides(&self,_: &Object);
}

impl ObjectProps for Object {
    fn draw(&self){
        match self {
            RIGID_BODY(r)=> r.draw()
        }
    }

    fn collides(&self,obj: &Object) {
        match self {
            RIGID_BODY(r)=> r.collides(obj)
        }
    }
}