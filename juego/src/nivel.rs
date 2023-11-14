// nivel.rs

use crate::habitacion::Habitacion;

const HABITACIONES_POR_NIVEL: usize = 2; // POR AHORA 2 PERO SON 5

pub struct Nivel{
    habitaciones: [Habitacion; HABITACIONES_POR_NIVEL],
}

impl Nivel {
    pub fn crear_nivel() -> Self{
        Nivel {
            let cantidad_habitaciones = 4;
            habitaciones: [Habitacion::inicializar_habitaciones_nivel(cantidad_habitaciones); HABITACIONES_POR_NIVEL],
        }

    }
}


pub fn crear_niveles(cantidad_niveles: usize) -> Vec<Nivel>{
    let mut niveles: Vec<Nivel> = Vec::new();

    for _ in 0..cantidad_niveles {
        let mut nivel = Nivel::crear_nivel();

        nivel.push(niveles);
    }

    niveles
}
