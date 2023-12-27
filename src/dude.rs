use std::any::Any;
use piston_window::*;

use crate::math::angle_for_down_vector;
use crate::{App, AppContext};
use crate::game_object::*;
use crate::math::*;
use crate::moon::*;
use crate::SCREEN_SIZE;
use crate::GRAVITY;
use crate::WALK_SPEED;
use crate::GROUND_Y;
use crate::SPRITE_SCALE;

#[derive(Default)]
 pub struct Dude {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    sprites: Vec<usize>,
    sprite_index: usize,
    sprite_size: [f64; 2],
    last_gravity: [f64; 2],
    flipped: bool,
    walking: bool,
    pub airborne: bool,
    pub on_moon: bool,
 }

 impl Dude {
    pub fn new() -> Dude {
        Dude {
            x: 80.0,
            y: 370.0,
            dx: 0.0,
            dy: 0.0,
            sprites: vec![0, 1, 2],
            sprite_index: 0,
            sprite_size: [16.0, 24.0],
            ..Default::default()
        }
    }

    pub fn center(&self) -> [f64; 2] {
        [ 
            self.x + (self.sprite_size[0] * SPRITE_SCALE) / 2.0, 
            self.y + (self.sprite_size[1] * SPRITE_SCALE) / 2.0 - 8.0  //??
        ]
    }

    pub fn radius(&self) -> f64 {
        (24.0 * SPRITE_SCALE) / 2.0
    }

    pub fn tangent_vector(&self) -> [f64; 2] {
        [ self.last_gravity[1], -self.last_gravity[0] ]
    }

    pub fn gravity_vector(&self) -> &[f64; 2] {
        &self.last_gravity
    }
    
    pub fn walk(&mut self, dx: f64) {
        let walk_vector = 
            if self.on_moon {
                normalized(&self.tangent_vector())
            } else {
                [ 1.0, 0.0 ]
            };
        
        if (dx > 0.0 && self.dx < 1.3) || (dx < 0.0 && self.dx > -1.3) {
            self.dx += dx * walk_vector[0] * 0.2;
            self.dy += dx * walk_vector[1] * 0.2;
        }
        self.flipped = dx < 0.0;
        self.walking = true;
    }

    pub fn jump(&mut self) {
        if self.airborne == true {
            //can't jump on air
            return;
        }

        let down = normalized(&self.last_gravity);
        self.dx = down[0] * -4.0;
        self.dy = down[1] * -4.0;
        self.airborne = true;        
    }

    fn time_to_frame(time: f64, time_scale: f64, num_frames: usize) -> usize {
        ((time*time_scale) % (num_frames as f64)) as usize
    }

    pub fn press(&mut self, context: &AppContext, args: &ButtonArgs) {
        if args.state != ButtonState::Press {
            return;
        }

        match args.button {
            Button::Keyboard(key) => {
                match key {
                    Key::Up | Key::W => {
                        self.jump();
                    }
                    _ => {}
                }
            }           
            _ => {} 
        }
    }     
 }

 impl GameObject for Dude {
    fn draw(&self, owner: &App, c: &Context, g: &mut G2d) {
        let size = [ self.sprite_size[0], self.sprite_size[1] ];

        let transform = c
            .transform
            .trans(self.x, self.y)
            .trans(size[0] / 2.0, size[1] / 2.0)
            .rot_rad(angle_for_down_vector(&self.last_gravity))
            .scale(if self.flipped { -SPRITE_SCALE } else { SPRITE_SCALE }, SPRITE_SCALE)
            .trans(-size[0] / 2.0, -size[1] / 2.0);
         //.trans(-25.0, -25.0);

        image(&owner.images.sprites[self.sprites[self.sprite_index]], transform, g);
    }

    fn update(&mut self, context: &AppContext, args: &UpdateArgs) {
        let height = SPRITE_SCALE*self.sprite_size[1] - 8.0;
        let dt = context.game_dt;

        self.x += self.dx * dt;
        self.y += self.dy * dt;

        self.dx += context.gravity[0] * dt;
        self.dy += context.gravity[1] * dt;
        self.last_gravity = context.gravity;

        if length(&[self.dx, self.dy]) > 3.0 {
            self.airborne = true;
        }

        if self.y+height > SCREEN_SIZE[1] - GROUND_Y {
            self.y = SCREEN_SIZE[1] - GROUND_Y - height;
            self.dy = 0.0;
            self.airborne = false;
        }

        if !self.airborne {
            self.dx *= (0.8 as f64).powf(dt * 2.0);
        }

        if self.airborne {
            if context.is_key_down(Button::Keyboard(Key::Up)) {
                //give a little jump hang time when holding up, mario-style
                //this is too much with distance attenuated gravity
                //self.dy -= GRAVITY * 0.45;
            }

            self.sprite_index = 2;
        } else if self.walking {
            let walk_frames = [ 0, 2 ];
            self.sprite_index = walk_frames[Self::time_to_frame(context.global_time, 6.0, 2)];
        } else {
            self.sprite_index = Self::time_to_frame(context.global_time, 2.0, 2)
        }

        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x + self.sprite_size[0] > SCREEN_SIZE[0] {
            self.x = SCREEN_SIZE[0] - self.sprite_size[0];
        }

        if context.is_key_down(Button::Keyboard(Key::Left)) {
            self.walk(-WALK_SPEED);
        } else if context.is_key_down(Button::Keyboard(Key::Right)) {
            self.walk(WALK_SPEED);
        } else {
            self.walking = false;
        }
    }

    fn bounds(&self) -> [f64; 4] {
        [ self.x, self.y, self.x+self.sprite_size[0], self.y+self.sprite_size[1] ]
    }

    fn as_any (&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }    
 }