use crate::{Object, ObjectTrait};

use super::App;
use graphics::clear;
use piston::RenderArgs;

pub struct PhysicsSim {
    pub app: App,
    pub objects: Vec<Object>,
}

impl PhysicsSim {
    pub fn new(app: App) -> Self {
        Self {
            app,
            objects: Default::default(),
        }
    }
}

impl PhysicsSim {
    pub fn draw(&mut self, args: &RenderArgs) {
        let context = self.app.gl.draw_begin(args.viewport());
        clear([0., 0., 0., 1.], &mut self.app.gl);
        //draw and check
        let len = self.objects.len();

        let time = 200.0;

        for i in 0..len {
            let mut iter = self.objects[i..len].iter_mut();
            let obj = iter.next().unwrap();

            for obj2 in iter {
                obj.check(obj2);
            }
            obj.calc_pos(time);
            obj.draw(&context, &mut self.app.gl);
        }
        //end draw
        self.app.gl.draw_end();
    }
}
