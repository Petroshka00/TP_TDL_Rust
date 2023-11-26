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
