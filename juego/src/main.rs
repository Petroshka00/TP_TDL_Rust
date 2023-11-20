use std::io;
//use std::cmp::Ordering;
//use std::ptr::null;
//use rand::Rng;
//use std::collections::HashMap;
use crossterm::{execute, terminal::{ClearType, Clear}}; // biblioteca para limpiar terminal

mod habitacion;
mod nivel;

use crate::{nivel::Nivel, habitacion::es_movimiento_valido};
use crate::habitacion::{ Jugador,Armadura, Arma, Posicion, imprimir_habitacion, inicializar_habitaciones_nivel};

const NIVELES_POR_JUEGO: usize = 1;// POR AHORA 1 PERO SON 5

pub struct Juego{
    niveles: Vec<Nivel>,
}

impl Juego {
    pub fn imprimir_datos_usuario(&self) {
        let jugador = &self.niveles[0].habitaciones[0].jugadores[0];
        let atributos = &jugador.atributos;

        println!("Nombre del jugador: {}", atributos.nombre);
        //println!("Posición del jugador: {:?}", atributos.posicion);
        println!("Salud actual/máxima: {}/{}", atributos.salud_actual, atributos.salud_max);
        println!("Daño: {}", atributos.daño);
        println!("Probabilidad de crítico: {}", atributos.prob_crit);
        println!("Armadura: {}", atributos.armadura);
        println!("Puntería: {}", atributos.punteria);
        println!("Esquiva: {}", atributos.esquiva);

        let estado_invisible = if atributos.invisible { "Sí" } else { "No" };
        println!("Invisible: {}", estado_invisible);

        println!("Arma equipada:");
        self.imprimir_datos_arma(&jugador.arma_equipada);

        println!("Armadura equipada:");
        self.imprimir_datos_armadura(&jugador.armadura_equipada);

        // Otros datos relacionados con el juego
    }

    fn imprimir_datos_arma(&self, arma: &Arma) {
        println!("Daño: {}", arma.daño);
        println!("Probabilidad de crítico: {}", arma.prob_crit);
        println!("Puntería: {}", arma.punteria);
    }

    fn imprimir_datos_armadura(&self, armadura: &Armadura) {
        println!("Armadura: {}", armadura.armadura);
        println!("Esquiva: {}", armadura.esquiva);
    }
}

fn crear_juego() -> Juego{
    
    //let mut niveles =  crear_niveles();
    let niveles = Nivel::crear_niveles(NIVELES_POR_JUEGO);

    let juego = Juego { niveles };

    juego

}


/* 
struct Trampa{
    daño: u32,
    efecto_especial: fn(jugador: Entidad), // Le aplica distintos efectos al jugador, ya sea modificar sus atributos u otras cosas
}
*/

/*fn es_critico(prob_critico: u32) -> bool{
    let chance_minima_necesaria: u32 = rand::thread_rng().gen_range(1..=100);
    if prob_critico >= chance_minima_necesaria {
        return true;
    }
    return false;
}

fn golpe(chance_de_golpe: u32, prob_critico: u32) -> u32{
    let chance_minima_necesaria: u32 = rand::thread_rng().gen_range(1..=100);

    if chance_de_golpe >= chance_minima_necesaria {
        if es_critico(prob_critico) {
            return 0;
        } else{
            return 1;
        }
    } else {
        return 2;
    }

}*/

/* 
    Combate entre dos entidades, disminuye el daño de ser necesario.
    Tiene en cuenta la punteria y el esquive.
*/
/*fn combate(mut ent_1: Atributos, mut ent_2: Atributos){
    let chance_de_golpe: u32 = ent_1.punteria - ent_2.esquiva;

    let resul_golpe  = golpe(chance_de_golpe, ent_1.prob_crit);

    match resul_golpe{
        0 => ent_2.salud_actual -= (ent_1.daño as f32 * 1.5) as u32, 
        1 => ent_2.salud_actual -= ent_1.daño - (ent_2.armadura / 100) * ent_1.daño,
        2 => println!("{} fallo su ataque!", ent_1.nombre),
        _ => return,
    }
}*/

fn movimiento(pos_actual: &mut Posicion, direccion: char){
    match direccion{
        'w' => pos_actual.y -= 1,
        's' => pos_actual.y += 1,
        'a' => pos_actual.x -= 1,
        'd' => pos_actual.x += 1,
        _ => return ,
    };
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


// Hace que el jugador pase de una habitacion a otra
/*fn pasar_de_habitacion(jugador: Jugador, habitacion1: &mut Habitacion, habitacion2: &mut Habitacion){
    habitacion1.jugadores.retain(|j| j.atributos.nombre != jugador.atributos.nombre);
    habitacion2.jugadores.push(jugador);
}*/

fn imprimir_tablero(juego: &mut Juego){

    for (nivel_numero, nivel) in juego.niveles.iter().enumerate() {
        println!("Nivel {}: ", nivel_numero);
        for habitacion in &nivel.habitaciones {
            imprimir_habitacion(habitacion);
            println!();
        }
    }
}


fn imprimir_mapa(juego: &mut Juego){
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap(); // borrar pantalla
    
    /*  
    println!(nivel , armadura, posciones)
    println! objetos inventario
    */ 
    imprimir_tablero(juego);

}


// Inicializa el juego, con todos sus niveles, habitaciones, etc y jugados / enemigos y objetos
fn inicializar_juego(juego: &mut Juego){

    let cantidad_habitaciones = 4;
   
    for nivel in juego.niveles.iter_mut() {
        nivel.habitaciones = inicializar_habitaciones_nivel(cantidad_habitaciones);
    }
    
    //enemigos --> generar_pos_en_hab
    //inicializar mapas

    // inicializar objetos
}

fn recibir_movimiento(juego: &mut Juego)-> bool{

    let mut input = String::new();

    println!("Presiona una tecla (w/a/s/d para mover, q para salir):");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    let direccion = input.trim().chars().next();



    if let Some(d) = direccion {
        let lim_superior = juego.niveles[0].habitaciones[0].dimension_y;
        let lim_lateral = juego.niveles[0].habitaciones[0].dimension_x;
        let nivel = &mut juego.niveles[0];
        let habitacion = &mut nivel.habitaciones[0];
        let jugador = &mut habitacion.jugadores[0];
        
        
        /*let posicion_anterior = jugador.atributos.posicion;*/

        match d {
            'w' => if es_movimiento_valido(jugador.atributos.posicion.y, -1, lim_superior){
                movimiento(&mut jugador.atributos.posicion, 'w');
            },
            's' => if es_movimiento_valido(jugador.atributos.posicion.y, 1, lim_lateral){
                movimiento(&mut jugador.atributos.posicion, 's');
            },
            'a' => if es_movimiento_valido(jugador.atributos.posicion.x, -1, 0){
                movimiento(&mut jugador.atributos.posicion, 'a');
            },
            'd' => if es_movimiento_valido(jugador.atributos.posicion.y, -1, 0){
                movimiento(&mut jugador.atributos.posicion, 'd');
            },
            'e' => {
                jugador.recoger_objeto(/*habitacion{actual} */)
            },
            'q' =>{
                print!("movimiento invalido");
                return false 
            } 
             _ => return true,
        }
    }

    true

}

fn main() {
    println!("Hello, world!");

    let mut juego = crear_juego();

    inicializar_juego(&mut juego);

    imprimir_mapa(&mut juego);

    loop {

        if! recibir_movimiento(&mut juego){ // recibir movimiento hace tab la jugada habria que modularizar
            break;
        }
        imprimir_mapa(&mut juego);

        (&mut juego).imprimir_datos_usuario();

    }
   
/* //LOGICA DEL MAIN PERO no toma en cuenta subir de nivel 

// 
    // while(estado_juego(juego)==ESTADO_JUGANDO)
    // recibir_movimiento();
    //realizar_jugada();
    //imprimir_mapa();

    
//if (estado_juego(juego)==ESTADO_GANADO){
   
//} else if (estado_juego(juego)==ESTADO_PERDIDO){
    
    */
}
