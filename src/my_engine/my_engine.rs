use crate::my_engine::{physics::*, app::*};

use piston::{RenderArgs, Event, RenderEvent};
use std::collections::{HashMap, HashSet};
use graphics::clear;

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
        let context = self.app.gl.draw_begin(args.viewport());
        clear([0.,0.,0.,0.],&mut self.app.gl);
        //draw and check
        {
            let mut values:Vec<&mut Box<Object>> = self.objects.values_mut().collect();
            let len = values.len();

            if len < 2 { return; }

            for x in 0..len {
                let mut l = values[x..len].iter_mut();
                let x = l.next().unwrap();
                for y in l {
                    x.check(y);
                }
                x.draw(&context, &mut self.app.gl);
            }
        }
        //end draw
        self.app.gl.draw_end();
    }
}