use std::any::Any;
use piston_window::*;

use crate::{App, AppContext};
use crate::game_object::*;
use crate::dude::*;
use crate::math::*;
use crate::SCREEN_SIZE;
use crate::GRAVITY;
use crate::GRAVITATIONAL_DISTANCE_SCALE;
use crate::SPRITE_SCALE;

const MOON_GRAVITY: f64 = 0.1;

pub struct Moon {
    x: f64,
    y: f64,
    radius: f64,
    gravity_scale: f64,
    circle: CircleArc,
}

impl Moon {
    pub fn new(x: f64, y: f64, radius: f64, gravity_scale: Option<f64>) -> Moon {
        let circle = CircleArc::new(
            [ 1.0, 1.0, 1.0, 1.0 ],
            1.0,
            0.0,
            2.0 * std::f64::consts::PI
        ).resolution(32);

        Moon {
            x, 
            y,
            radius,
            gravity_scale: gravity_scale.unwrap_or(1.0),
            circle
        }
    }

    pub fn attract(&self, dude: &Dude, gravity: &mut [f64; 2]) {
        let dude_center = dude.center();
        let vector = [ self.x - dude_center[0], self.y - dude_center[1] ];
        let distance = length(&vector) * GRAVITATIONAL_DISTANCE_SCALE;
        let vector: [f64; 2] = normalized(&vector);

        //fun > realism
        let attraction = (1.0 / (distance*distance)) * MOON_GRAVITY * self.gravity_scale;
        let attraction = attraction.clamp(0.0, 1.0);

        gravity[0] += attraction * vector[0];
        gravity[1] += attraction * vector[1];
    }

    pub fn collide(&self, dude: &mut Dude) -> bool {
        let dude_center = dude.center();
        let vector = [ self.x - dude_center[0], self.y - dude_center[1] ];
        let distance = length(&vector);
        let vector: [f64; 2] = normalized(&vector);

        if distance < dude.radius() + self.radius {
            let adj_distance = distance - (dude.radius() + self.radius);

            let dx = vector[0] * adj_distance;
            let dy = vector[1] * adj_distance;

            dude.x += dx;
            dude.y += dy;
            dude.dx *= 0.8;
            dude.dy *= 0.8;
            dude.airborne = false;

            return true;
        }

        false
    }
}

impl GameObject for Moon {    
    fn draw(&self, owner: &App, c: &Context, g: &mut G2d) {
        let x = self.x;
        let y = self.y;
        let radius = self.radius;
        let sprite_sz = owner.images.sprites[3].get_size();
        let sprite_sz: (f64, f64) = (sprite_sz.0 as f64, sprite_sz.1 as f64);
        let scale = ((radius * 2.0) / sprite_sz.0 as f64) * 1.05;

        //so moony..
        let transform = c
            .transform
            .trans(x - sprite_sz.0 / 2.0 - 1.0,  y - sprite_sz.1 / 2.0)
            .trans(sprite_sz.0 / 2.0, sprite_sz.1 / 2.0)
            .scale(scale, scale)
            .trans(-sprite_sz.0 / 2.0, -sprite_sz.1 / 2.0);
        image(&owner.images.sprites[3], transform, g);    
        
        let ds = DrawState::default();
        self.circle.draw([ x - radius, y - radius, radius*2.0, radius*2.0], &ds, c.transform, g);
    }

    fn update(&mut self, context: &AppContext, args: &UpdateArgs) {

    }

    fn bounds(&self) -> [f64; 4] {
        [ self.x-self.radius, self.y-self.radius, self.x+self.radius, self.y+self.radius ]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }    
}