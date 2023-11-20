// nivel.rs

use crate::habitacion::Habitacion;

const HABITACIONES_POR_NIVEL: usize = 2; // POR AHORA 2 PERO SON 5

pub struct Nivel{
    pub habitaciones: Vec<Habitacion>,
}

impl Nivel {
    /*pub fn crear_nivel() -> Self {
        let habitaciones = vec![Habitacion::crear_habitacion(); HABITACIONES_POR_NIVEL];
        Nivel { habitaciones }
    }*/

    pub fn crear_niveles(num_niveles: usize) -> Vec<Nivel> {
        let mut niveles = Vec::with_capacity(num_niveles);

        for _ in 0..num_niveles {
            let habitaciones = Habitacion::crear_habitaciones(HABITACIONES_POR_NIVEL);
            niveles.push(Nivel { habitaciones });
        }

        niveles
    }
}



