use crate::my_engine::{physics::* ,app::*};

use piston::RenderArgs;

pub struct MyEngine<'a,T> where T: Object {
    pub app :App,
    pub objects:Vec<&'a Box<T>>
}

impl<T:Object> MyEngine<'_,T> {
    pub fn new(app:App)->MyEngine<'static,T> {
        MyEngine {
            app,
            objects : Vec::new()
        }
    }
    pub fn draw(&mut self,args:&RenderArgs){
        self.app.gl.draw(args.viewport(), | c, gl| {

        })
    }
}