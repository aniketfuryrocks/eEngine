use crate::my_engine::{physics::*, app::*};

use piston::{RenderArgs, Event, RenderEvent};
use std::collections::{HashMap, HashSet};

pub struct MyEngine {
    pub app: App,
    pub objects: HashMap<String, Box<Object>>,
}

impl MyEngine {
    pub fn new(app: App) -> MyEngine {
        MyEngine {
            app,
            objects: HashMap::new(),
        }
    }

    pub fn draw(&mut self, args: &RenderArgs) {
        self.app.gl.draw(args.viewport(), |c, gl| {
            /*for (_, x) in &self.objects {
                for (_, y) in & self.objects {
                    //dont detect collision for itself
                  /*  if &x == &y {
                        continue;
                    }*/
                    x.collides(y);
                }
            }*/
        })
    }
}