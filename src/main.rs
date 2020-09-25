mod my_engine;
use my_engine::*;
use opengl_graphics::*;
use piston::*;
use crate::my_engine::physics::{RigidBody, RigidShape};
use crate::my_engine::math::Vector2D;

//const
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1., 1., 1., 1.];
const BG:[f32; 4] = WHITE;

//logic
fn main() {
    let mut eng = MyEngine::new(
        App::new(AppSettings {
            window_settings: WindowSettings::new("AwsmGame", [200, 200]).exit_on_esc(true),
            gl_ver: OpenGL::V4_5,
        })
    );

    let mut events = Events::new(EventSettings::new());

    let player = Box::new(RigidBody {
        pos : Vector2D {x:2.,y:2.},
        shape : RigidShape::RECTANGLE
    });
    //add a rigid body
    eng.objects.push(&player);

    &player.shape;

    while let Some(e) = events.next(&mut eng.app.window) {
        if let Some(args) = e.render_args() {
            eng.draw(&args);
        }
    }
}
