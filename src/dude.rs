use piston_window::*;

use crate::{App, AppContext};
use crate::SCREEN_SIZE;
use crate::GRAVITY;
use crate::WALK_SPEED;
use crate::GROUND_Y;
use crate::SPRITE_SCALE;

#[derive(Default)]
 pub struct Dude {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    sprites: Vec<usize>,
    sprite_index: usize,
    sprite_size: [f64; 2],
    flipped: bool,
    walking: bool,
    jumping: bool,
 }

 impl Dude {
    pub fn new() -> Dude {
        Dude {
            x: 300.0,
            y: 300.0,
            dx: 0.0,
            dy: 0.0,
            sprites: vec![0, 1, 2],
            sprite_index: 0,
            sprite_size: [16.0, 24.0],
            ..Default::default()
        }
    }

    pub fn draw(&self, owner: &App, c: &Context, g: &mut G2d) {
        let size = [ self.sprite_size[0], self.sprite_size[1] ];

        let transform = c
            .transform
            .trans(self.x, self.y)
            .trans(size[0] / 2.0, size[1] / 2.0)
            .scale(if self.flipped { -SPRITE_SCALE } else { SPRITE_SCALE }, SPRITE_SCALE)
            .trans(-size[0] / 2.0, -size[1] / 2.0);
         //.rot_rad(rotation)
         //.trans(-25.0, -25.0);

        image(&owner.images.sprites[self.sprites[self.sprite_index]], transform, g);

        //draw a primitive ground line
        line_from_to([1.0, 1.0, 1.0, 1.0], 1.0, 
            [0.0, SCREEN_SIZE[1]-GROUND_Y], [SCREEN_SIZE[0], SCREEN_SIZE[1]-GROUND_Y], 
            c.transform, g);
    }
    
    pub fn walk(&mut self, dx: f64) {
        self.x += dx;
        self.flipped = dx < 0.0;
        self.walking = true;
    }

    pub fn jump(&mut self) {
        if self.jumping == true {
            //can't jump on air
            return;
        }

        self.jumping = true;
        self.dy = -4.0;
    }

    fn time_to_frame(time: f64, time_scale: f64, num_frames: usize) -> usize {
        ((time*time_scale) % (num_frames as f64)) as usize
    }

    pub fn update(&mut self, context: &AppContext, args: &UpdateArgs) {
        let height = SPRITE_SCALE*self.sprite_size[1] - 8.0;

        self.x += self.dx;
        self.y += self.dy;
        self.dy += GRAVITY;

        if self.y+height > SCREEN_SIZE[1] - GROUND_Y {
            self.y = SCREEN_SIZE[1] - GROUND_Y - height;
            self.dy = 0.0;
            self.jumping = false;
        }

        if self.jumping {
            if context.is_key_down(Button::Keyboard(Key::Up)) {
                //give a little jump hang time when holding up, mario-style
                self.dy -= GRAVITY * 0.45;
            }

            self.sprite_index = 2;
        } else if self.walking {
            let walk_frames = [ 0, 2 ];
            self.sprite_index = walk_frames[Self::time_to_frame(context.global_time, 6.0, 2)];
        } else {
            self.sprite_index = Self::time_to_frame(context.global_time, 2.0, 2)
        }

        if context.is_key_down(Button::Keyboard(Key::Left)) {
            self.walk(-WALK_SPEED);
        } else if context.is_key_down(Button::Keyboard(Key::Right)) {
            self.walk(WALK_SPEED);
        } else {
            self.walking = false;
        }
    }

    pub fn press(&mut self, context: &AppContext, args: &ButtonArgs) {
        if args.state != ButtonState::Press {
            return;
        }

        match args.button {
            Button::Keyboard(key) => {
                match key {
                    Key::Up => {
                        self.jump();
                    }
                    _ => {}
                }
            }           
            _ => {} 
        }
    }     
 }
