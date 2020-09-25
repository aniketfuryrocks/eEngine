use crate::my_engine::{physics::* ,app::*};

use piston::{RenderArgs, Event, RenderEvent};
use std::collections::{HashMap, HashSet};

pub struct MyEngine<T> where T: Object {
    pub app :App,
    pub objects:HashMap<String,Box<T>>
}

impl<T:Object> MyEngine<T> {
    pub fn new(app:App)->MyEngine<T> {
        MyEngine {
            app,
            objects : HashMap::new()
        }
    }

    pub fn draw(&mut self,args:&RenderArgs){
        self.app.gl.draw(args.viewport(), | c, gl| {

        })
    }
}