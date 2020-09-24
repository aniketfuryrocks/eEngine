mod my_engine;
use my_engine::*;
use opengl_graphics::*;
use piston::*;

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
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut eng.app.window) {
        if let Some(args) = e.render_args() {
            eng.app.gl.draw(args.viewport(), |c,gl| {
                use graphics::*;
            
                let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
                
                clear(GREEN, gl);

                let transform = c
                    .transform
                    .trans(x, y)
                    .rot_rad(3.12/2.0)
                    .trans(-25.0, -25.0);

                // Draw a box rotating around the middle of the screen.
                rectangle(BG, rectangle::square(0.0, 0.0, 50.0), transform, gl);
            })
        }
    }
}
