use e_engine::{eEngine, math::Vector2D, physics::RigidBody, shape::Rectangle, App, AppSettings};
use opengl_graphics::OpenGL;
use piston::{EventSettings, Events, WindowSettings};

//const
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1., 1., 1., 1.];
//const BG: [f32; 4] = WHITE;

//logic
fn main() {
    let mut eng = eEngine::new(App::new(AppSettings {
        window_settings: WindowSettings::new("Earth-Moon simulation", [700, 800]).exit_on_esc(true),
        gl_ver: OpenGL::V4_5,
    }));

    // earth
    eng.objects.push(
        RigidBody {
            shape: Rectangle {
                width: 20.,
                height: 20.,
                color: GREEN,
            }
            .into(),
            center: Vector2D { x: 350.0, y: 350.0 },
            mass: 6.673e11,
            velocity: 1.,
        }
        .into(),
    );

    // moon
    eng.objects.push(
        RigidBody {
            shape: Rectangle {
                width: 3.,
                height: 3.,
                color: WHITE,
            }
            .into(),
            center: Vector2D { x: 500.4, y: 400.4 },
            mass: 6.673e9,
            velocity: 1.,
        }
        .into(),
    );

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut eng.app.window) {
        use piston::RenderEvent;
        if let Some(args) = e.render_args() {
            eng.draw(&args);
        }
    }
}
