use rltk::{VirtualKeyCode, Rltk, Point};
use specs::prelude::*;
use std::cmp::{max, min};
use crate::{MAPWIDTH, MAPHEIGHT, Item, gamelog::GameLog, WantsToPickupItem, TileType};

use super::{Position, Player, Viewshed, State, Map, RunState, CombatStats, WantsToMelee};

/// Intenta mover al jugador si es posible
pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, viewshed) in (&entities, &players, &mut positions, &mut viewsheds).join() {
        if pos.x + delta_x < 1 || pos.x + delta_x > map.width-1 || pos.y + delta_y < 1 || pos.y + delta_y > map.height-1 { return; }
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target }).expect("No se pudo agregar el objetivo");
                return;
            }
        }

        if !map.blocked[destination_idx] {
            pos.x = min(MAPWIDTH as i32 - 1, max(0, pos.x + delta_x));
            pos.y = min(MAPHEIGHT  as i32 - 1, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item : Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => gamelog.entries.push("No hay nada aqui para recoger".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup.insert(*player_entity, WantsToPickupItem{ collected_by: *player_entity, item }).expect("No se pudo insertar");
        }
    }
}

pub fn try_next_level(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    if map.tiles[player_idx] == TileType::DownStairs {
        true
    } else {
        let mut gamelog = ecs.fetch_mut::<GameLog>();
        gamelog.entries.push("No se puede bajar por aqui.".to_string());
        false
    }
}

/// Lee el input del usuario
pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput } // No se pulsa nada
        Some(key) => match key { // se pulsa algo
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            
            VirtualKeyCode::E => get_item(&mut gs.ecs),

            VirtualKeyCode::I => return RunState::ShowInventory,
            VirtualKeyCode::T => return RunState::ShowDropItem,
            VirtualKeyCode::R => return RunState::ShowRemoveItem,

            VirtualKeyCode::X => {
                if try_next_level(&mut gs.ecs) {
                    return RunState::NextLevel;
                }
            }
            
            _ => { return RunState::AwaitingInput }
        },
    }
    RunState::PlayerTurn
}