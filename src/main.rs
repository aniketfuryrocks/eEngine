mod my_engine;
use my_engine::*;
use opengl_graphics::*;
use piston::*;
use graphics::*;
use std::thread::sleep;
use std::time::Duration;

//const
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1., 1., 1., 1.];
const BG:[f32; 4] = WHITE;

//logic
fn main() {
    let mut eng = MyEngine::new(
        App::new(AppSettings {
            window_settings: WindowSettings::new("awsm_game", [200, 200]).exit_on_esc(true),
            gl_ver: OpenGL::V4_5,
        })
    );
    let mut events = {
        let mut set =  EventSettings::new();
        set.set_max_fps(2000);
        Events::new(set)
    };
    let mut pos = 0.;
    let mut dir = 1.;
    while let Some(e) = events.next(&mut eng.app.window) {
        if let Some(args) = e.render_args() {
            let view_port = args.viewport();
            eng.app.gl.draw(view_port, |c,gl| {
                clear(BG, gl);
                eng.draw(&c, gl);
                Line::new(GREEN,2.)
                    .draw([pos,pos,pos+30.,pos+30.],&c.draw_state, c.transform,gl);
                if pos > view_port.window_size[0]||pos > view_port.window_size[1] {
                    dir = -1.;
                } else if pos < 0. {
                    dir = 1.;
                }
                pos = pos + (5. * dir);
                println!("{},{}",pos,dir);
            });
        }
    }
}
