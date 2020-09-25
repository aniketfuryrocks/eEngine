use crate::my_engine::{physics::* ,app::*};

use piston::RenderArgs;

pub struct MyEngine<T> where T: Object {
    pub app :App,
    pub objects:Vec<Box<T>>
}

impl<T:Object> MyEngine<T> {
    pub fn new(app:App)->MyEngine<T> {
        MyEngine {
            app,
            objects : Vec::new()
        }
    }

    pub fn spawn_mut(&mut self,obj:T)->&mut Box<T>{
        let len = self.objects.len();
        self.objects.push(Box::new(obj));
        &mut self.objects[len]
    }

    pub fn spawn(&mut self, obj:T) ->& Box<T>{
        let len = self.objects.len();
        self.objects.push(Box::new(obj));
        &self.objects[len]
    }

    pub fn draw(&mut self,args:&RenderArgs){
        /*self.app.gl.draw(args.viewport(), | _c, _gl| {

        })*/
    }
}