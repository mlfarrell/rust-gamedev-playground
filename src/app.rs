
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;
use std::error::Error;
use piston_window::*;
use piston::input::{RenderArgs, UpdateArgs};

use crate::images::Images;
use crate::dude::Dude;
use crate::game_object::*;
use crate::moon::*;

pub const SCREEN_SIZE: [f64; 2] = [ 640.0, 480.0 ];
pub const SPRITE_SCALE: f64 = 2.0;
pub const WALK_SPEED: f64 = 2.0;
pub const GROUND_Y: f64 = 32.0;
pub const GRAVITY: f64 = 0.08;
pub const GRAVITATIONAL_DISTANCE_SCALE: f64 = 0.008;
//pub const GRAVITATIONAL_DISTANCE_SCALE: f64 = 0.01;

pub struct AppContext {
    pub global_time: f64,
    pub game_dt: f64,
    pub key_state: HashSet<Button>,
    pub gravity: [f64; 2],
    pub scroll: [f64; 2],
 }

 impl AppContext {
    pub fn key_alias_for(button: Key) -> Option<Key> {
        if button == Key::Up { Some(Key::W) }
        else if button == Key::Left { Some(Key::A) }
        else if button == Key::Down { Some(Key::S) }
        else if button == Key::Right { Some(Key::D) }
        else { None }
    }

    pub fn is_key_down(&self, button: Button) -> bool {
        if let Button::Keyboard(key) = button {
            if let Some(aliased) = Self::key_alias_for(key) {
                if self.key_state.contains(&Button::Keyboard(aliased))  {
                    return true;
                }
            }
        }

        self.key_state.contains(&button)
    }
 }
pub struct App {
    pub images: Images,
    dude: Box<Dude>,
    moons: Vec<Box<Moon>>,
    context: AppContext,
}

impl App {
    pub fn new(images: Images) -> Result<Self, Box<dyn Error>> {        
        Ok(Self {
            images: images,
            dude: Box::new(Dude::new()),
            moons: Self::place_moons(),
            context: AppContext {
                global_time: 0.0,
                game_dt: 0.0,
                key_state: HashSet::new(),
                gravity: [0.0, 0.0],
                scroll: [0.0, 0.0],
            }
        })
    }

    fn place_moons() -> Vec<Box<Moon>> {
        let moons: Vec<Moon> = vec![
            Moon::new(320.0, 300.0, 60.0, None),
            Moon::new(320.0, 100.0, 40.0, None),
            Moon::new(460.0, 200.0, 30.0, None),
            Moon::new(520.0, 50.0, 35.0, None),
            Moon::new(410.0, -125.0, 70.0, Some(1.5)),
            Moon::new(605.0, -115.0, 15.0, Some(0.5)),
            Moon::new(25.0, -50.0, 10.0, None),
            Moon::new(200.0, -80.0, 12.0, None),
            Moon::new(40.0, -150.0, 13.0, None),
            Moon::new(50.0, -250.0, 20.0, None),
        ];

        moons.into_iter().map(|m| {
            Box::new(m)
        }).collect()
    }

    pub fn resize(&mut self, window: &mut PistonWindow, args: &ResizeArgs) {
    }

    pub fn render(&mut self, e: &Event, window: &mut PistonWindow, args: &RenderArgs) {
        use graphics::*;

        window.draw_2d(e, |c, g, _| {
            //let sz = c.get_view_size();
            //let scale = [sz[0] / 640.0, sz[1] / 480.0];
            let c = c.trans(-self.context.scroll[0], -self.context.scroll[1]);

            clear([0.1, 0.1, 0.1, 1.0], g);

            let transform = c.transform
                .trans(0.0, -(self.images.sprites[4].get_size().1 as f64) + SCREEN_SIZE[1]);
            image(&self.images.sprites[4], transform, g);

            self.dude.draw(&self, &c, g);
            for obj in self.moons.iter() {
                obj.draw(&self, &c, g);
            }

            //draw a primitive ground line
            rectangle([1.0, 1.0, 1.0, 1.0], [0.0, SCREEN_SIZE[1]-GROUND_Y, SCREEN_SIZE[0], GROUND_Y], c.transform, g);            
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
        self.context.game_dt = args.dt * 60.0;       
        self.context.gravity = [ 0.0, 0.0 ];

        for m in self.moons.iter() {
            m.attract(&self.dude, &mut self.context.gravity);
        }

        //ground gravity
        let dist_from_ground = (self.dude.y - (SCREEN_SIZE[1]-GROUND_Y)).abs() * GRAVITATIONAL_DISTANCE_SCALE;
        let atten = (1.0 / (dist_from_ground * dist_from_ground)).clamp(0.0, 1.0);
        self.context.gravity[1] += GRAVITY * atten;

        self.dude.update(&self.context, &args);

        self.dude.on_moon = false;
        for m in self.moons.iter() {
            if m.collide(&mut self.dude) {
                self.dude.on_moon = true;
            }
        }

        //vertical scrolling
        self.context.scroll[1] = self.dude.y - SCREEN_SIZE[1]/2.0;
        if self.context.scroll[1] > 0.0 {
            self.context.scroll[1] = 0.0;
        } 
        if self.context.scroll[1] < -893.0 + SCREEN_SIZE[1] {
            self.context.scroll[1] = -893.0 + SCREEN_SIZE[1];
        }
    }
}
