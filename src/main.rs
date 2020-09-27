mod my_engine;

use opengl_graphics::*;
use piston::*;
use crate::my_engine::math::Vector2D;
use crate::my_engine::*;
use graphics::clear;
use crate::my_engine::physics::{RigidBody, Object, RigidShape, Rectangle};

//const
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1., 1., 1., 1.];
const BG: [f32; 4] = WHITE;

//logic
fn main() {
    let mut eng = MyEngine::new(
        App::new(AppSettings {
            window_settings: WindowSettings::new("AwsmGame", [700, 800]).exit_on_esc(true),
            gl_ver: OpenGL::V4_5,
        })
    );
    //add a rigid body
    eng.objects.insert("earth".to_string(), Box::new(Object::RigidBody(RigidBody {
        shape: RigidShape::RECTANGLE(Rectangle {
            width: 3.,
            height: 3.,
            color: GREEN
        }),
        center: Vector2D {
            x: 350.0,
            y: 350.0
        },
        mass: 6.673e11,
        velocity: 1.
    })));
    eng.objects.insert("moon".to_string(), Box::new(Object::RigidBody(RigidBody {
        shape: RigidShape::RECTANGLE(Rectangle {
            width: 3.,
            height: 3.,
            color: WHITE
        }),
        center: Vector2D {
            x: 500.4,
            y: 400.4
        },
        mass: 6.673e9,
        velocity: 1.
    })));
    /*eng.objects.insert("ground".to_string(), Box::new(Object::RigidBody(RigidBody {
        shape: RigidShape::RECTANGLE(Rectangle {
            width: 1400.,
            height: 2.,
            color: [1.,1.,1.,1.]
        }),
        center: Vector2D {
            x: 400.0,
            y: 700.0
        },
        mass: 2.0e9,
        velocity: 0.
    })));
*/
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut eng.app.window) {
        if let Some(args) = e.render_args() {
           /* if let Object::RigidBody(player) = eng.objects.get_mut("player_1").unwrap().as_mut() {
               player.center.x+=2.;
            }*/
            /*if let Object::RigidBody(player) = eng.objects.get_mut("player_2").unwrap().as_mut() {
                player.center.y+=5.;
                player.center.x-=1.;

            }*/
            eng.draw(&args);
        }
    }
}