extern crate piston_window;

mod dude;
mod images;
mod app;

use piston_window::*;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use app::*;
use images::Images;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow =
        WindowSettings::new("Gamey", [SCREEN_SIZE[0], SCREEN_SIZE[1]])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let tex_context = window.create_texture_context();
    let tex_settings = TextureSettings::new()
        .filter(Filter::Nearest);

    let mut images = Images::new(tex_context, tex_settings);
    images.load().unwrap();

    let mut app = App::new(images).unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&e, &mut window, &args);
        }

        if let Some(args) = e.button_args() {            
            app.press(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }        
}
