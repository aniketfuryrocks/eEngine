use glutin_window::GlutinWindow;
use piston::window::{WindowSettings};
use opengl_graphics::{GlGraphics,OpenGL};

pub struct AppSettings{
    pub window_settings: WindowSettings,
    pub gl_ver: OpenGL
}

pub struct App {
    pub window : GlutinWindow,
    pub gl:GlGraphics
}

impl App {
    pub fn new(settings:AppSettings)->App{
        App {
            window : settings.window_settings.graphics_api(settings.gl_ver).build().unwrap(),
            gl: GlGraphics::new(settings.gl_ver)
        }
    }
}