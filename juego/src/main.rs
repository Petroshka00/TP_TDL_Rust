use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

struct Juego{
    niveles: [Nivel; 5],
}

struct Nivel{
    habitaciones: [Habitacion; 5],
}

struct Habitacion{
    dimension_x : u8,
    dimension_y : u8,
    puertas: [Posicion; 4],
    entidades: Entidades,
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
    let pos_x_gen: u8 = rand::thread_rng().gen_range(1..=habitacion.dimension_x);
    let pos_y_gen: u8 = rand::thread_rng().gen_range(1..=habitacion.dimension_y);

    let posicion: Posicion = Posicion {
        pos_x : pos_x_gen,
        pos_y : pos_y_gen,
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

// Hace que el jugador (O entidad, a chequear) pase de una habitacion a otra
fn pasar_de_habitacion(jugador: Jugador, habitacion1: Habitacion, habitacion2: Habitacion){

}

// Inicializa las habitaciones de un nivel, con sus objetos, enemigos, tamaño, etc
fn inicializar_habitaciones_nivel(nivel: Nivel){

}

// Inicializa el juego, con todos sus niveles, habitaciones, etc
fn inicializar_juego(juego: Juego){

}

fn main() {
    println!("Hello, world!");
}
