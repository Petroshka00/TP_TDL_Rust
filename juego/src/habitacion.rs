use rand::Rng;

/*const PUERTAS_POR_HABITACION: usize = 4;*/
const CANT_DE_RECOGIBLES: usize = 7; 


pub struct Habitacion{
    pub dimension_x : u32,
    pub dimension_y : u32,
    pub puertas: Vec<Puerta>,
    pub jugadores: Vec<Jugador>,
    pub enemigos: Vec<Enemigo>,
    pub objetos_suelo : Vec<Recogible>,
}

pub struct Recogible{
    pub consumible : TipoObjeto, 
    pub posicion: Posicion,
}

impl Recogible {

    pub fn crear_un_objeto_random(habitacion: &mut Habitacion) -> TipoObjeto {
        let mut rng = rand::thread_rng();
        let opcion = rng.gen_range(0..2);
    
    
        match opcion {
            0 => TipoObjeto::Arma(Arma::new(
                rng.gen_range(5..15), // Daño aleatorio entre 5 y 14
                rng.gen_range(5..15), // Probabilidad de crítico aleatoria entre 5 y 14
                rng.gen_range(5..15), // Puntería aleatoria entre 5 y 14
            )),
            1 => TipoObjeto::Armadura(Armadura::new(
                rng.gen_range(5..15), // Armadura aleatoria entre 5 y 14
                rng.gen_range(5..15), // Esquiva aleatoria entre 5 y 14
            )),
            _ => unreachable!(),
        }
    }

    pub fn crear_vector_recogible(habitacion: &mut Habitacion, num_recogibles: usize) -> Vec<Recogible> {
        let mut recogibles = Vec::with_capacity(num_recogibles);

        for _ in 0..num_recogibles {
            let posicion = generar_pos_en_hab(habitacion);
            let consumible = Recogible::crear_un_objeto_random(habitacion);
    
            recogibles.push(Recogible { consumible, posicion });
        }
    
        recogibles
    }
    

}

pub struct Puerta{
    pub posicion: Posicion,
    pub desde_hab: u32,
    pub hasta_hab: u32,
}

enum Entidades{
    Jugador(Jugador),
    Objetos(TipoObjeto),
    Enemigos(Enemigo),
}

pub struct Posicion{
    pub x: u32,    
    pub y: u32,
}
pub enum TipoObjeto {
    Arma(Arma),
    /*Pocion(Pocion),*/
    Armadura(Armadura),
}

pub struct Arma{
    pub daño: u32,
    pub prob_crit: u32,
    pub punteria: u32,
}

impl Arma {
    pub fn new(daño: u32, prob_crit: u32, punteria: u32) -> Self {
        Arma { daño, prob_crit, punteria }
    }
}

/*struct Pocion{
    duracion: u32,
    funcionalidad: fn(),
}*/

/*enum TipoPocion{
    Vida,
    Fuerza,
    PielDeHierro,
    Invisibilidad,
    Punteria,
    Esquiva,
}*/

/*impl Pocion{
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
}*/

pub struct Armadura{
    pub armadura: u32,
    pub esquiva: u32,
}

impl Armadura {
    pub fn new(armadura: u32, esquiva: u32) -> Self {
        Armadura { armadura, esquiva }
    }
}

pub struct Atributos{
    pub nombre: String,
    pub posicion: Posicion,
    pub salud_max: u32,
    pub salud_actual: u32,
    pub daño: u32,
    pub prob_crit: u32,
    pub armadura: u32,
    pub punteria: u32,
    pub esquiva: u32,
    pub invisible: bool,
}

pub struct Jugador{
    pub atributos: Atributos,
    pub arma_equipada: Arma,
    pub armadura_equipada: Armadura,
    /*inventario: Vec<TipoObjeto>,*/
}

pub struct Enemigo{

}



impl Habitacion {

    pub fn crear_habitaciones(cantidad: usize) -> Vec<Habitacion> {
        let mut habitaciones = Vec::with_capacity(cantidad);
    
        for _ in 0..cantidad {
            let habitacion = Habitacion::crear_habitacion();
            habitaciones.push(habitacion);
        }
    
        habitaciones
    }

    pub fn crear_habitacion() -> Self {
        Habitacion {
            dimension_x: 10,
            dimension_y: 10,
            puertas:vec! [
                Puerta {
                    posicion: Posicion { x: 1, y: 2 },
                    desde_hab: 0,
                    hasta_hab: 1,
                },
                Puerta {
                    posicion: Posicion { x: 5, y: 7 },
                    desde_hab: 0,
                    hasta_hab: 2,
                },
                Puerta {
                    posicion: Posicion { x: 8, y: 3 },
                    desde_hab: 1,
                    hasta_hab: 0,
                },
                Puerta {
                    posicion: Posicion { x: 2, y: 8 },
                    desde_hab: 2,
                    hasta_hab: 0,
                },
            ],
           
            jugadores: Vec::new(),
            enemigos: Vec::new(),
            objetos_suelo: Vec::new(),
        }

    }


}



/* 
fn recoger_pocion(pocion: Pocion, jugador: Jugador){
    jugador.inventario.append(pocion);
}
*/


impl Jugador {
    pub fn recoger_objeto(&mut self/*,  objetos_suelo: &Vec<Recogible>*/){
        
        /*for objeto in objetos_suelo {
            if objeto.posicion == self.atributos.posicion {
                self.equipar_objeto(objeto.clone());
                println!("Has recogido un objeto.");
                return; 
            }
        }*/
    
        println!("No hay un objeto para recoger en tu posición.");
    }
    fn equipar_objeto(&mut self, objeto: TipoObjeto) {
        match objeto {
            TipoObjeto::Arma(arma) => self.arma_equipada = arma,
            TipoObjeto::Armadura(armadura) => self.armadura_equipada = armadura,
        }
    }
    /*fn usar_pocion(&mut self, indice_pocion: usize){
        
    }*/
}



// Esto genera una posicion aleatoria dentro de las dimensiones de la habitacion que recibe para colocar los objetos y enemigos al inicializar
fn generar_pos_en_hab(habitacion: &mut Habitacion) -> Posicion{
    // let x_gen: u32 = rand::thread_rng().gen_range(1..=habitacion.dimension_x);
    // let y_gen: u32 = rand::thread_rng().gen_range(1..=habitacion.dimension_y);

    let posicion: Posicion = Posicion {
        x : rand::thread_rng().gen_range(1..=habitacion.dimension_x),
        y : rand::thread_rng().gen_range(1..=habitacion.dimension_y),
    };

    return posicion;
}

fn esta_tocando_puerta(habitacion: Habitacion) -> bool{
    return true;
}


fn generar_dimensiones_hab(habitacion: &mut Habitacion) -> &mut Habitacion{
    habitacion.dimension_x = rand::thread_rng().gen_range(4..=10);
    habitacion.dimension_y = rand::thread_rng().gen_range(4..=10);

    return habitacion;
}

fn generar_puertas(habitacion: &mut Habitacion) -> &mut Habitacion {
    for mut puertas in &mut habitacion.puertas{
           
        let puerta = Puerta { 
            posicion: Posicion { x: 1, y: 2 },  // falta esa logica para que sea random
            desde_hab: 0, 
            hasta_hab: 1,
        };
    }
    habitacion
}

// Inicializa las habitaciones de un nivel, con sus objetos, enemigos, tamaño, etc
pub fn inicializar_habitaciones_nivel(cantidad_habitaciones: usize) -> Vec<Habitacion>{
    let mut habitaciones: Vec<Habitacion> = Vec::new();

    for _ in 0..cantidad_habitaciones {
        let mut habitacion = Habitacion::crear_habitacion();
        generar_dimensiones_hab(&mut habitacion);
        generar_puertas(&mut habitacion); 
        habitacion.objetos_suelo = Recogible::crear_vector_recogible(&mut habitacion, CANT_DE_RECOGIBLES);
        inicializar_jugador(&mut habitacion);

        habitaciones.push(habitacion);
    }

    habitaciones
}

pub fn imprimir_habitacion(habitacion: &Habitacion) {


    println!("Dimensiones: {}x{}", habitacion.dimension_x, habitacion.dimension_y);
    let mut matriz: Vec<Vec<String>> = vec![vec!["-".to_string(); habitacion.dimension_x as usize]; habitacion.dimension_y as usize]; // creamos una matriz de tamaño x y 
 

    for jugador in &habitacion.jugadores {
        if jugador.atributos.posicion.x < habitacion.dimension_x && jugador.atributos.posicion.y < habitacion.dimension_y {
            matriz[jugador.atributos.posicion.y as usize][jugador.atributos.posicion.x as usize] = "J".to_string();
        }
    }

    for fila in matriz.iter() {
        for celda in fila.iter() {
        print!("{}  ", celda);
        }
        println!();
    }

    for recogible in &habitacion.objetos_suelo {
        if recogible.posicion.x < habitacion.dimension_x && recogible.posicion.y < habitacion.dimension_y {
            matriz[recogible.posicion.y as usize][recogible.posicion.x as usize] = "R".to_string();
        }
    }


   // println!("Puertas: {:?}", habitacion.puertas);
   // println!("Jugadores: {:?}", habitacion.jugadores);
}


fn inicializar_jugador(habitacion: &mut Habitacion) -> &mut Habitacion{

    let jugador = Jugador {
        atributos: Atributos {
            nombre: "String".to_string(), 
            posicion: generar_pos_en_hab( habitacion),
            salud_max: 100,
            salud_actual: 100,
            daño: 100,
            prob_crit: 0,
            armadura: 0,
            punteria: 0,
            esquiva: 0,
            invisible: false,
        },
        arma_equipada: Arma {
            daño: 0,
            prob_crit: 0,
            punteria: 0,
        },
        armadura_equipada: Armadura {
            armadura: 0,
            esquiva: 0,
        },
        /*inventario: Vec::new(), // Inicializa el inventario como un vector vacío*/
    };
 
    habitacion.jugadores.push(jugador);

    return habitacion;

   
}


pub fn es_movimiento_valido(pos_en_eje: u32, mov_direccion: i32, limite_habitacion: u32) -> bool {
    return (pos_en_eje as i32) + mov_direccion != (limite_habitacion as i32);
}

