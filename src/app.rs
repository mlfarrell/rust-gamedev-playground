
use std::collections::HashSet;
use std::error::Error;
use piston_window::*;
use piston::input::{RenderArgs, UpdateArgs};

use crate::images::Images;
use crate::dude::Dude;

pub const SCREEN_SIZE: [f64; 2] = [ 640.0, 480.0 ];
pub const SPRITE_SCALE: f64 = 2.0;
pub const WALK_SPEED: f64 = 2.0;
pub const GROUND_Y: f64 = 32.0;
pub const GRAVITY: f64 = 0.08;

pub struct AppContext {
    pub global_time: f64,
    pub key_state: HashSet<Button>
 }

 impl AppContext {
    pub fn is_key_down(&self, button: Button) -> bool {
        self.key_state.contains(&button)
    }
 }

pub struct App {
    pub images: Images,
    dude: Dude,
    context: AppContext,
}

impl App {
    pub fn new(images: Images) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            images: images,
            dude: Dude::new(),
            context: AppContext {
                global_time: 0.0,
                key_state: HashSet::new(),
            }
        })
    }

    pub fn render(&mut self, e: &Event, window: &mut PistonWindow, args: &RenderArgs) {
        use graphics::*;

        window.draw_2d(e, |c, g, _| {
            clear([0.1, 0.1, 0.1, 1.0], g);

            self.dude.draw(&self, &c, g);
        });
    }

    pub fn press(&mut self, args: &ButtonArgs) {
        if args.state == ButtonState::Press {
            self.context.key_state.insert(args.button);
        } else {
            self.context.key_state.remove(&args.button);
        }

        self.dude.press(&self.context, &args);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.context.global_time += args.dt;        
        self.dude.update(&self.context, &args);
    }
}
