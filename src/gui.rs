use rltk::{ RGB, Rltk, Console };
use specs::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const GUI_WIDTH : usize = SCREEN_WIDTH;
const GUI_HEIGHT : usize = 6;


pub fn draw_ui(ecs: &World, ctx : &mut Rltk) {
    ctx.draw_box(0, SCREEN_HEIGHT - GUI_HEIGHT - 1, GUI_WIDTH - 1, GUI_HEIGHT, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
}
