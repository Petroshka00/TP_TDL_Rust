use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

struct Juego{
    niveles: [Nivel; ],

}

struct Nivel{
    habitaciones: [Habitacion; ],

}

struct Habitacion{
    dimensiones: [[u8; ]; ],
    puertas: [[Posicion; ]; ],
}

struct Posicion{
    pos_x: u8,    
    pos_y: u8,
}

/* 
struct Objeto{
    posicion_en_inventario: u8,
    equipado: bool,
}
*/

struct Arma{
    daño: u8,
    punteria: u8,
}

struct Pocion{
    funcionalidad: fn(),
    duracion: u8,
}

struct Armadura{
    armadura: u8,
    esquiva: u8,
}

struct Entidad{
    posicion: Posicion,
    salud_max: u8,
    salud_actual: u8,
    daño: u8,
    armadura: u8,
    punteria: u8,
    esquiva: u8,
}

struct Jugador{
    atributos: Entidad,
    arma_equipada: Arma,
    armadura_equipada: Armadura,
    herramienta_equipada: Herramienta,
    inventario: Vec<Objeto>, // Esto tiene que ser un vector/array de distintos objetos/armas, hay que revisar
}

/* 
struct Trampa{
    daño: u8,
    efecto_especial: fn(jugador: Entidad), // Le aplica distintos efectos al jugador, ya sea modificar sus atributos u otras cosas
}
*/

/* Combate entre dos entidades, disminuye el daño de ser necesario.
    Tiene en cuenta la punteria y el esquive.
 */
fn combate(mut ent_1: Entidad, mut ent_2: Entidad){
    let chance_de_golpe: u8 = ent_1.punteria - ent_2.esquiva;
    let chance_minima_necesaria: u8 = rand::thread_rng().gen_range(1..=100);

    match chance_de_golpe.cmp(&chance_minima_necesaria){
        Ordering::Less => println!("Le erro"), // No le pega directamente
        Ordering::Equal => println!("Cosas"), //Le dio justo
        Ordering::Greater => ent_2.salud = ent_2.salud - ent_1.daño, // Le hace el daño que deberia, aca agregar algo con la armadura
    }
}

// Esto genera una posicion aleatoria dentro de las dimensiones de la habitacion que recibe
fn generar_pos_en_hab(habitacion: Habitacion){

}

// Esto le otorga el arma en el piso al jugador cuando apreta la tecla de recoger
fn recoger_arma(arma: Arma, jugador: Jugador){

}

// Esto le otorga la armadura en el piso al jugador cuando apreta la tecla de recoger
fn recoger_armadura(armadura: Armadura, jugador: Jugador){

}

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
