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
                width: 30.,
                height: 30.,
                color: GREEN,
            }
            .into(),
            center: Vector2D { x: 350.0, y: 350.0 },
            mass: 6.673e11,
            velocity: 0.,
        }
        .into(),
    );

    // moon
    eng.objects.push(
        RigidBody {
            shape: Rectangle {
                width: 10.,
                height: 10.,
                color: WHITE,
            }
            .into(),
            center: Vector2D { x: 400.4, y: 600.4 },
            mass: 6.673e9,
            velocity: 0.2,
        }
        .into(),
    );

    // moon 2
    eng.objects.push(
        RigidBody {
            shape: Rectangle {
                width: 10.,
                height: 10.,
                color: [0.4, 0.1, 0.3, 1.],
            }
            .into(),
            center: Vector2D { x: 700.4, y: 500.4 },
            mass: 6.673e8,
            velocity: 3.,
        }
        .into(),
    );
    
    // moon 3
    eng.objects.push(
        RigidBody {
            shape: Rectangle {
                width: 10.,
                height: 10.,
                color: [0.9, 0.2, 0.5, 1.],
            }
            .into(),
            center: Vector2D { x: 200., y: 200. },
            mass: 6.673e8,
            velocity: 2.,
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
