use crate::{MAPWIDTH};

use super::{CombatStats, Player, Renderable, Name, Position, Viewshed, Monster, BlocksTile, Rect, Item
    , EquipmentSlot, Equippable, MeleePowerBonus, DefenseBonus };
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;

const MAX_MONSTERS : i32 = 4;
const MAX_ITEMS : i32 = 5;

/// Spawnea el jugador y devuelve su entidad
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('P'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order : 0,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build()
}

/// Spawnea un monstruo en una posicion dada
pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 20);
    }
    match roll {    // Asegurarse que la generacion de monstruos comprende TODOS los valores de la funcion de roll_dice
        1..=3 => orc(ecs, x, y),
        4..=10 => goblin(ecs, x, y),
        11..=16 => slime(ecs, x, y),
        17..=20 => rat(ecs, x, y),
        _ => print!("Esto no deberia suceder!")
    }
}

fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('O'), "Orco", 16, 16, 1, 4);
}
fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin", 10, 10, 0, 3);
}
fn slime(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('s'), "Slime", 6, 6, 0, 2);
}
fn rat(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('r'), "Rata", 4, 4, 0, 2);
}

fn monster<S: ToString>(
    ecs: &mut World,
    x: i32,
    y: i32,
    glyph: rltk::FontCharType,
    name: S,
    max_hp: i32,
    hp: i32,
    defense: i32,
    power: i32,
) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order : 1,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp,
            hp,
            defense,
            power,
        })
        .build();
}

pub fn random_equipment(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 20);
    }
    match roll {    // Asegurarse que la generacion de monstruos comprende TODOS los valores de la funcion de roll_dice
        1 => espada(ecs, x, y),
        2 => daga(ecs, x, y),
        3 => martillo(ecs, x, y),
        4 => espadon(ecs, x, y),
        5 => tablones_de_madera(ecs, x, y),
        6 => escudo(ecs, x, y),
        7 => gran_escudo(ecs, x, y),
        8 => escudo_con_puas(ecs, x, y),
        9 => casco_simple(ecs, x, y),
        10 => armadura_simple(ecs, x, y),
        _ => print!("Esto no deberia suceder!")
    }
}

fn espada(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('e'), "Espada", EquipmentSlot::Weapon, 0, 3) }

fn daga(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('d'), "Daga", EquipmentSlot::Weapon, 0, 2) }

fn martillo(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('m'), "Martillo", EquipmentSlot::Weapon, 0, 4) }

fn espadon(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('E'), "Espadon", EquipmentSlot::Weapon, 1, 5) }

fn tablones_de_madera(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('|'), "Tablones de madera", EquipmentSlot::Shield, 1, 0) }

fn escudo(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('('), "Escudo", EquipmentSlot::Shield, 2, 0) }

fn gran_escudo(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('['), "Gran escudo", EquipmentSlot::Shield, 3, 0) }

fn escudo_con_puas(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('{'), "Escudo con puas", EquipmentSlot::Shield, 2, 1) }

fn casco_simple(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('^'), "Casco simple", EquipmentSlot::Helmet, 1, 0) }

fn armadura_simple(ecs: &mut World, x: i32, y: i32){ equipment(ecs, x, y, rltk::to_cp437('%'), "Armadura simple", EquipmentSlot::Armor, 3, 0) }

fn equipment<S: ToString>(ecs: &mut World,
    x: i32,
    y: i32,
    glyph: rltk::FontCharType,
    name: S,
    slot : EquipmentSlot,
    defense: i32,
    power: i32,){
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : name.to_string() })
        .with(Item{})
        .with(Equippable{ slot })
        .with(DefenseBonus{ defense })
        .with(MeleePowerBonus { power })
        .build();
}

pub fn spawn_room(ecs: &mut World, room : &Rect) {
    let mut monster_spawn_points : Vec<usize> = Vec::new();
    let mut weapon_spawn_points : Vec<usize> = Vec::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        
        for _i in 0 .. num_monsters {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        let num_weapons = rng.roll_dice(1, MAX_ITEMS + 4) - 3;

        for _i in 0 .. num_weapons {
            let mut added = false;
            while !added{
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !weapon_spawn_points.contains(&idx) {
                    weapon_spawn_points.push(idx);
                    added = true;
                }

            }
        }
    }

    for idx in monster_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(ecs, x as i32, y as i32);
    }

    for idx in weapon_spawn_points.iter(){
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_equipment(ecs, x as i32, y as i32);
    }
}