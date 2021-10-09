use crate::{Object, ObjectTrait};

use super::App;
use graphics::clear;
use piston::RenderArgs;

#[allow(non_camel_case_types)]
pub struct eEngine {
    pub app: App,
    pub objects: Vec<Object>,
}

impl eEngine {
    pub fn new(app: App) -> Self {
        Self {
            app,
            objects: Default::default(),
        }
    }
}

impl eEngine {
    pub fn draw(&mut self, args: &RenderArgs) {
        let context = self.app.gl.draw_begin(args.viewport());
        clear([0., 0., 0., 1.], &mut self.app.gl);
        //draw and check
        let len = self.objects.len();

        for i in 0..len {
            let mut iter = self.objects[i..len].iter_mut();
            let obj = iter.next().unwrap();
            let time = 1.0 / 25.0;

            for obj2 in iter {
                obj.check(obj2, time);
            }
            obj.calc_pos(time);
            obj.draw(&context, &mut self.app.gl);
        }
        //end draw
        self.app.gl.draw_end();
    }
}
