use rltk::{ RGB, Rltk, RandomNumberGenerator, BaseMap, Algorithm2D, Point };

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, gui};
use gui::GUI_HEIGHT;

use super::{Rect};
use std::cmp::{max, min};
use specs::prelude::*;

pub const MAPWIDTH : usize = SCREEN_WIDTH;
pub const MAPHEIGHT : usize = SCREEN_HEIGHT - GUI_HEIGHT;
const MAPCOUNT : usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor, DownStairs
}

#[derive(Clone)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : i32,
    pub height : i32,
    pub revealed_tiles : Vec<bool>,
    pub visible_tiles : Vec<bool>,
    pub blocked : Vec<bool>,
    pub tile_content : Vec<Vec<Entity>>,
    pub depth : i32,
}

impl Map {
    /// Esta funcion sirve para encontrar el indice en el vector/array de la posicion xy
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    /// Genera un espacio vacio o habitacion en el mapa
    fn apply_room_to_map(&mut self, room : &Rect) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// Genera espacios vacios entre 2 puntos horizontales
    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        for x in min(x1,x2) ..= max(x1,x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Genera espacios vacios entre 2 puntos verticales
    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1,y2) ..= max(y1,y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Toma un indice del mapa y calcula si se puede mover ahi o no
    fn is_tile_valid_for_movement(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { return false; }
        let idx = self.xy_idx(x, y);
        self.tiles[idx as usize] != TileType::Wall
    }

    // Registra los espacios bloqueados
    pub fn populate_blocked(&mut self) {
        for (i,tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }


    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    /// Genera un mapa nuevo, con habitaciones y pasillos
    pub fn new_map_rooms_and_corridors(new_depth: i32) -> Map {
        let mut map = Map{
            tiles : vec![TileType::Wall; MAPCOUNT],
            rooms : Vec::new(),
            width : MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles : vec![false; MAPCOUNT],
            visible_tiles : vec![false; MAPCOUNT],
            blocked : vec![false; MAPCOUNT],
            tile_content : vec![Vec::new(); MAPCOUNT],
            depth: new_depth
        };

        const MAX_ROOMS : i32 = 30;     // Estos parametros pueden cambiarse segun el mapa
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
                    if rng.range(0,2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }
        let stairs_position = map.rooms[map.rooms.len()-1].center();
        let stairs_idx = map.xy_idx(stairs_position.0, stairs_position.1);
        map.tiles[stairs_idx] = TileType::DownStairs;

        map
    }
}

impl BaseMap for Map {
    /// Hace que el mapa este completamente lleno de paredes, para despues ser vaciado para generar habitaciones, etc
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    /// Pathfinding desde un punto hacia otro
    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    /// Devuelve los posibles movimientos a partir de una posicion en el mapa
    fn get_available_exits(&self, idx:usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;
    
        // Cardinal directions
        if self.is_tile_valid_for_movement(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_tile_valid_for_movement(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_tile_valid_for_movement(x, y-1) { exits.push((idx-w, 1.0)) };
        if self.is_tile_valid_for_movement(x, y+1) { exits.push((idx+w, 1.0)) };
    
        exits
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

/// Dibuja el mapa en la consola
pub fn draw_map(ecs: &World, ctx : &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx,tile) in map.tiles.iter().enumerate() {
        // Dibuja el tipo de "tile" correspondiente

        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0.4, 0.6, 0.5);
                }
                TileType::DownStairs => {
                    glyph = rltk::to_cp437('↓');
                    fg = RGB::from_f32(1.0, 1.0, 0.);
                }
            }
            if !map.visible_tiles[idx] { fg = fg.to_greyscale() }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
        }

        // Esto llega hasta 80 porque la pantalla mide 80, cambiar segun necesidad
        x += 1;
        if x > SCREEN_WIDTH - 1 {
            x = 0;
            y += 1;
        }
    }
}