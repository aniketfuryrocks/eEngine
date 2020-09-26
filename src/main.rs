mod my_engine;

use opengl_graphics::*;
use piston::*;
use crate::my_engine::physics::{RigidBody, RigidShape, Object, Rectangle};
use crate::my_engine::math::Vector2D;
use graphics::line::Shape::Square;
use crate::my_engine::*;
use graphics::clear;

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
    let colors = [WHITE,GREEN];
    //add a rigid body
    for k in 0..colors.len() {
        eng.objects.insert(format!("player_{}",k), Box::new(Object::RigidBody(RigidBody {
            shape: RigidShape::RECTANGLE(Rectangle {
                pos: Vector2D {
                    x: 50.* (k) as f64,
                    y: 50.* (k) as f64 *5 as f64,
                },
                width: 50.,
                height: 50.,
                color: colors[k]
            })
        })));
    }

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut eng.app.window) {
        if let Some(args) = e.render_args() {
            if let Object::RigidBody(player) = eng.objects.get_mut("player_1").unwrap().as_mut() {
                if let RigidShape::RECTANGLE(s) = &mut player.shape {
                    s.pos.x+=1.;
                   // s.
                }
            }
            eng.draw(&args);
        }
    }
}