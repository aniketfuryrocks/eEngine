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
    eng.objects.insert("player_1".to_string(), Box::new(Object::RigidBody(RigidBody {
        shape: RigidShape::RECTANGLE(Rectangle {
            width: 50.,
            height: 50.,
            color: WHITE
        }),
        center: Vector2D {
            x: 25.0,
            y: 25.0
        },
        mass: 20.0e3,
        velocity: 1.0
    })));
    eng.objects.insert("player_2".to_string(), Box::new(Object::RigidBody(RigidBody {
        shape: RigidShape::RECTANGLE(Rectangle {
            width: 50.,
            height: 50.,
            color: GREEN
        }),
        center: Vector2D {
            x: 200.0,
            y: 200.0
        },
        mass: 2.0,
        velocity: 1.0
    })));

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut eng.app.window) {
        if let Some(args) = e.render_args() {
            if let Object::RigidBody(player) = eng.objects.get_mut("player_1").unwrap().as_mut() {
                if let RigidShape::RECTANGLE(s) = &mut player.shape {
//                    s.pos.x+=1.;
                   // s.
                }
            }
            eng.draw(&args);
        }
    }
}