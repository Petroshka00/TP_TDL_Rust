const socket = new WebSocket("ws://localhost:8080");

socket.addEventListener("open", (event) => {
    console.log("Conexión establecida");
});

socket.addEventListener("message", (event) => {
    const message = event.data;
    console.log("Mensaje recibido:", message);
});

socket.addEventListener("close", (event) => {
    console.log("Conexión cerrada");
});

socket.addEventListener("error", (event) => {
    console.error("Error en la conexión:", event);
});

// Ejemplo de enviar un mensaje al servidor
document.addEventListener("keydown", (event) => {
    const keyCode = event.keyCode;
    socket.send(JSON.stringify({ action: "move", keyCode: keyCode }));
});
