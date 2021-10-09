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
                width: 10.,
                height: 10.,
                color: GREEN,
            }
            .into(),
            center: Vector2D { x: 300.0, y: 300.0 },
            mass: 4.659e4,
            velocity: Default::default(),
        }
        .into(),
    );

    // sun
    eng.objects.push(
        RigidBody {
            shape: Rectangle {
                width: 10.,
                height: 10.,
                color: GREEN,
            }
            .into(),
            center: Vector2D { x: 600.0, y: 300.0 },
            mass: 1.989e6,
            velocity: Default::default(),
        }
        .into(),
    );

    // sun
    eng.objects.push(
        RigidBody {
            shape: Rectangle {
                width: 10.,
                height: 10.,
                color: GREEN,
            }
            .into(),
            center: Vector2D { x: 100.0, y: 200.0 },
            mass: 9.989e5,
            velocity: Default::default(),
        }
        .into(),
    );

    for k in 1..20 {
        eng.objects.push(
            RigidBody {
                shape: Rectangle {
                    width: 10.,
                    height: 10.,
                    color: WHITE,
                }
                .into(),
                center: Vector2D {
                    x: 300.0 + (k * if k % 2 == 0 { -1 * k } else { k }) as f64,
                    y: 350.038 + k as f64,
                },
                mass: 7.34e2 * k as f64,
                velocity: Vector2D {
                    x: -1.022e-20,
                    y: 0.0,
                },
            }
            .into(),
        );
    }

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut eng.app.window) {
        use piston::RenderEvent;
        if let Some(args) = e.render_args() {
            eng.draw(&args);
        }
    }
}
