use rltk::{ RGB, Rltk, Console };
use specs::prelude::*;

use super::{CombatStats, Player};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, gamelog::GameLog};

pub const GUI_WIDTH : usize = SCREEN_WIDTH;
pub const GUI_HEIGHT : usize = 10;


pub fn draw_ui(ecs: &World, ctx : &mut Rltk) {
    ctx.draw_box(0, SCREEN_HEIGHT - GUI_HEIGHT - 1, GUI_WIDTH - 1, GUI_HEIGHT, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    let log = ecs.fetch::<GameLog>();

    let mut y = SCREEN_HEIGHT - GUI_HEIGHT + 1;
    for s in log.entries.iter().rev() {
        if y < SCREEN_HEIGHT - 1 { ctx.print(2, y, s); }
        y += 1;
    }

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(SCREEN_WIDTH / 8, SCREEN_HEIGHT - GUI_HEIGHT - 1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);

        ctx.draw_bar_horizontal(SCREEN_WIDTH / 4, SCREEN_HEIGHT - GUI_HEIGHT - 1, 51, stats.hp, stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));
    }
}

