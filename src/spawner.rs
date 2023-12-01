use super::{BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, Viewshed};
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;

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
    match roll {
        1..=3 => orc(ecs, x, y),
        4..=10 => goblin(ecs, x, y),
        11..=16 => slime(ecs, x, y),
        17..=20 => rat(ecs, x, y),
        _ => print!("Esto no deberia suceder!")
    }
}

fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('O'), "Orc", 16, 16, 1, 4);
}
fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin", 10, 10, 0, 3);
}
fn slime(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('s'), "Slime", 6, 6, 0, 2);
}
fn rat(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('r'), "Rat", 4, 4, 0, 2);
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
