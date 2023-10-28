use std::io;
use std::cmp::Ordering;
use std::ptr::null;
use rand::Rng;
use std::collections::HashMap;

const NIVELES_POR_JUEGO: usize = 5;
const HABITACIONES_POR_NIVEL: usize = 5;
const PUERTAS_POR_HABITACION: usize = 4;
const INDICE_JUGADOR: usize = 0;

struct Juego{
    niveles: [Nivel; NIVELES_POR_JUEGO],
}

struct Nivel{
    habitaciones: [Habitacion; HABITACIONES_POR_NIVEL],
}

struct Habitacion{
    dimension_x : u8,
    dimension_y : u8,
    puertas: [Puerta; PUERTAS_POR_HABITACION],
    jugadores: Vec<Jugador>,
    enemigos: Vec<Enemigo>,
    objetos_suelo : Vec<TipoObjeto>,
}

struct Puerta{
    posicion: Posicion,
    desde_hab: u8,
    hasta_hab: u8,
}

enum Entidades{
    Jugador(Jugador),
    Objetos(TipoObjeto),
    Enemigos(Enemigo),
}

struct Posicion{
    pos_x: u8,    
    pos_y: u8,
}

enum TipoObjeto {
    Arma(Arma),
    Pocion(Pocion),
    Armadura(Armadura),
}

struct Arma{
    daño: u8,
    prob_crit: u8,
    punteria: u8,
}
struct Pocion{
    duracion: u8,
    funcionalidad: fn(),
}

enum TipoPocion{
    Vida,
    Fuerza,
    PielDeHierro,
    Invisibilidad,
    Punteria,
    Esquiva,
}

impl Pocion{
    fn vida(jugador: &mut Jugador){
        jugador.atributos.salud_actual += 40;
    }

    fn fuerza(jugador: &mut Jugador){
        jugador.atributos.daño += 10;
    }

    fn pieldehierro(jugador: &mut Jugador){
        jugador.atributos.armadura += 50;
    }

    fn invisibilidad(jugador: &mut Jugador){
        jugador.atributos.invisible = true;
    }

    fn punteria(jugador: &mut Jugador){
        jugador.atributos.punteria += 20;
    }

    fn esquiva(jugador: &mut Jugador){
        jugador.atributos.esquiva += 30;
    }
}

struct Armadura{
    armadura: u8,
    esquiva: u8,
}

struct Atributos{
    nombre: String,
    posicion: Posicion,
    salud_max: u8,
    salud_actual: u8,
    daño: u8,
    prob_crit: u8,
    armadura: u8,
    punteria: u8,
    esquiva: u8,
    invisible: bool,
}

struct Jugador{
    atributos: Atributos,
    arma_equipada: Arma,
    armadura_equipada: Armadura,
    inventario: Vec<TipoObjeto>, 
}

struct Enemigo{

}

impl Jugador {
    fn recoger_objeto(&mut self, objeto: TipoObjeto) {
        self.inventario.push(objeto)
    }
    fn usar_pocion(&mut self, indice_pocion: usize){
        
    }
}

/* 
struct Trampa{
    daño: u8,
    efecto_especial: fn(jugador: Entidad), // Le aplica distintos efectos al jugador, ya sea modificar sus atributos u otras cosas
}
*/

fn es_critico(prob_critico: u8) -> bool{
    let chance_minima_necesaria: u8 = rand::thread_rng().gen_range(1..=100);
    if(prob_critico >= chance_minima_necesaria){
        return true;
    }
    return false;
}

fn golpe(chance_de_golpe: u8, prob_critico: u8) -> u8{
    let chance_minima_necesaria: u8 = rand::thread_rng().gen_range(1..=100);

    if(chance_de_golpe >= chance_minima_necesaria){
        if(es_critico(prob_critico)){
            return 0;
        } else{
            return 1;
        }
    } else {
        return 2;
    }

}

/* Combate entre dos entidades, disminuye el daño de ser necesario.
    Tiene en cuenta la punteria y el esquive.
 */
fn combate(mut ent_1: Atributos, mut ent_2: Atributos){
    let chance_de_golpe: u8 = ent_1.punteria - ent_2.esquiva;

    let resul_golpe = golpe(chance_de_golpe, ent_1.prob_crit);

    match resul_golpe{
        0 => ent_2.salud_actual -= (ent_1.daño as f32 * 1.5) as u8, 
        1 => ent_2.salud_actual -= ent_1.daño - (ent_2.armadura / 100) * ent_1.daño,
        2 => println!("{} fallo su ataque!", ent_1.nombre),
        _ => return,
    }
}

fn movimiento(pos_actual: &mut Posicion, direccion: char){
    match direccion{
        'w' => pos_actual.pos_y -= 1,
        's' => pos_actual.pos_y += 1,
        'a' => pos_actual.pos_x -= 1,
        'd' => pos_actual.pos_x += 1,
        _ => return,
    };
}

// Esto genera una posicion aleatoria dentro de las dimensiones de la habitacion que recibe para colocar los objetos y enemigos al inicializar
fn generar_pos_en_hab(habitacion: Habitacion) -> Posicion{
    // let pos_x_gen: u8 = rand::thread_rng().gen_range(1..=habitacion.dimension_x);
    // let pos_y_gen: u8 = rand::thread_rng().gen_range(1..=habitacion.dimension_y);

    let posicion: Posicion = Posicion {
        pos_x : rand::thread_rng().gen_range(1..=habitacion.dimension_x),
        pos_y : rand::thread_rng().gen_range(1..=habitacion.dimension_y),
    };

    return posicion;
}

/* 
// Esto le otorga el arma en el piso al jugador cuando apreta la tecla de recoger
fn recoger_arma(arma: Arma, jugador: Jugador){
    jugador.inventario.append(arma);
}

// Esto le otorga la armadura en el piso al jugador cuando apreta la tecla de recoger
fn recoger_armadura(armadura: Armadura, jugador: Jugador){
    jugador.inventario.append(armadura);
}

fn recoger_pocion(pocion: Pocion, jugador: Jugador){
    jugador.inventario.append(pocion);
}
*/

fn esta_tocando_puerta(habitacion: Habitacion) -> bool{
    return true;
}

// Hace que el jugador pase de una habitacion a otra
fn pasar_de_habitacion(jugador: Jugador, habitacion1: &mut Habitacion, habitacion2: &mut Habitacion){
    habitacion1.jugadores.retain(|j| j.atributos.nombre != jugador.atributos.nombre);
    habitacion2.jugadores.push(jugador);
}

fn generar_dimensiones_hab(habitacion: &mut Habitacion) -> &mut Habitacion{
    habitacion.dimension_x = rand::thread_rng().gen_range(4..=10);
    habitacion.dimension_y = rand::thread_rng().gen_range(4..=10);

    return habitacion;
}

// Inicializa las habitaciones de un nivel, con sus objetos, enemigos, tamaño, etc
fn inicializar_habitaciones_nivel(nivel: &mut Nivel){
    for (mut habitacion) in &mut nivel.habitaciones{
        habitacion = generar_dimensiones_hab(habitacion);
    }
}

// Inicializa el juego, con todos sus niveles, habitaciones, etc
fn inicializar_juego(juego: &mut Juego){

}

fn main() {
    println!("Hello, world!");
}
