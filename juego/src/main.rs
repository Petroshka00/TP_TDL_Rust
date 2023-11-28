use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::server::accept_async;
use tokio_tungstenite::WebSocketStream;
use tokio::sync::{mpsc, Mutex};
use std::collections::HashMap;
//use std::sync::{Arc, Mutex};

struct Jugador {
    id: usize,
    stream: WebSocketStream<TcpStream>,
}

use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
use visibility_system::VisibilitySystem;

pub struct State {
    pub ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind address");
    println!("WebSocket server listening on: {}", addr);

    let (tx, _) = mpsc::channel::<Message>(100);

    let players = Mutex::new(HashMap::new());

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let players = players.clone();

        tokio::spawn(handle_connection(stream, tx, players));
    }
}

async fn handle_connection(
    stream: TcpStream,
    tx: mpsc::Sender<Message>,
    players: Mutex<HashMap<usize, Jugador>>,
) {
    if let Ok(ws_stream) = accept_async(stream).await {
        let (tx_player, rx_player) = mpsc::channel::<Message>(100);

        let player_id = {
            let mut players = players.lock().await;
            let player_id = players.len();
            players.insert(player_id, Jugador { id: player_id, stream: ws_stream.clone() });
            player_id
        };

        ///let players = Arc::new(Mutex::new(players)); //?
        let tx_clone = tx.clone();
        tokio::spawn(receive_messages(player_id, rx_player, tx_clone, players.clone()));

        // Enviar bienvenida al jugador
        tx.send(Message::text(format!("¡Bienvenido Jugador {}!", player_id))).await.unwrap();

        // Lógica del juego
        game_logic(player_id, tx_player, players.clone()).await;

        // Remover al jugador cuando la conexión se cierra
        players.lock().await.remove(&player_id);
    }
}

async fn receive_messages(
    player_id: usize,
    mut rx: mpsc::Receiver<Message>,
    tx: mpsc::Sender<Message>,
    players: Mutex<HashMap<usize, Jugador>>,
) {
    while let Some(msg) = rx.recv().await {
        // Aquí puedes manejar los mensajes recibidos, por ejemplo, enviarlos a otros jugadores
        let mut players = players.lock().await;
        for (id, jugador) in players.iter() {
            if *id != player_id {
                jugador.stream.send(msg.clone()).await.unwrap();
            }
        }
    }
}

async fn game_logic(
    player_id: usize,
    mut tx: mpsc::Sender<Message>,
    players: Mutex<HashMap<usize, Jugador>>,
) {
    // Lógica del juego (adaptada para el uso con WebSockets)
    loop {
        // Puedes utilizar la función recibir_movimiento o adaptar tu lógica actual
        let movimiento = recibir_movimiento(/* parámetros necesarios */);
        
        // Ejemplo: enviar el movimiento a los otros jugadores
        let mut players = players.lock().await;
        for (id, jugador) in players.iter() {
            if *id != player_id {
                jugador.stream.send(Message::text(format!("Jugador {} se movió: {:?}", player_id, movimiento))).await.unwrap();
            }
        }

        // Puedes ajustar la frecuencia de actualización según tus necesidades
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

fn recibir_movimiento(/* parámetros necesarios */) {
    // Implementa tu función recibir_movimiento aquí o utiliza tu lógica actual
    // ...
}


// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

/* ESTO ES EL MAIN PARA EL JUEGO SINGLEPLAYER, falta adaptar a MP.
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .build();

    rltk::main_loop(context, gs)
}
*/