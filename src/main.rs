mod my_engine;

use my_engine::*;
use opengl_graphics::*;
use piston::*;
use crate::my_engine::physics::{RigidBody, RigidShape};
use crate::my_engine::math::Vector2D;
use graphics::line::Shape::Square;

//const
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1., 1., 1., 1.];
const BG: [f32; 4] = WHITE;

//logic
fn main() {
    let mut eng = MyEngine::new(
        App::new(AppSettings {
            window_settings: WindowSettings::new("AwsmGame", [200, 200]).exit_on_esc(true),
            gl_ver: OpenGL::V4_5,
        })
    );
    //add a rigid body
    eng.objects.insert("player".to_string(), Box::new(RigidBody {
        pos: Vector2D { x: 2., y: 2. },
        shape: RigidShape::RECTANGLE,
    }));

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut eng.app.window) {
        if let Some(args) = e.render_args() {
            let mut player = eng.objects.get_mut("player").unwrap();
            player.shape = RigidShape::ELLIPSE;
            /*eng.draw(&args);
            player
            println!("{:?}", player.shape)*/
        }
    }
}