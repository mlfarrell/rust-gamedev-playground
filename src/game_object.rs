use std::any::Any;
use piston_window::*;
use crate::{App, AppContext};

pub trait GameObject: Any {
    fn bounds(&self) -> [f64; 4];
    fn update(&mut self, context: &AppContext, args: &UpdateArgs);
    fn draw(&self, owner: &App, c: &Context, g: &mut G2d);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

