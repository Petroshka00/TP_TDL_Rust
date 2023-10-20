use std::io;
use std::cmp::Ordering;
use rand::Rng;

struct Juego{
    mapa: [[i32; 10]; 10], // A decidir el tamaño
}

struct Posicion{
    pos_x: u8,    
    pos_y: u8,
}

struct Objeto{
    posicion_en_inventario: u8,
    equipado: bool,

}

struct Arma{
    daño: u8,
    punteria: u8,
}

struct Herramienta{
    funcionalidad: fn(),
    durabilidad: u8,
}

struct Armadura{
    armadura: u8,
    esquiva: u8,
}

struct Entidad{
    posicion: Posicion,
    salud: u8,
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
    inventario: Vec<Objeto>,
}

struct Trampa{
    daño: u8,
    efecto_especial: fn(jugador: Entidad),
}

fn combate(mut ent_1: Entidad, mut ent_2: Entidad){
    let chance_de_golpe: u8 = ent_1.punteria - ent_2.esquiva;
    let chance_minima_necesaria: u8 = rand::thread_rng().gen_range(1..=100);

    match chance_de_golpe.cmp(&chance_minima_necesaria){
        Ordering::Less => println!("Le erro"), // No le pega directamente
        Ordering::Equal => println!("Cosas"), //Le dio justo, capaz un golpe critico?
        Ordering::Greater => ent_2.salud = ent_2.salud - ent_1.daño, // Le hace el daño que deberia, aca agregar algo con la armadura
    }



}

fn inicializar_mapa(){

}


fn main() {
    println!("Hello, world!");
}
