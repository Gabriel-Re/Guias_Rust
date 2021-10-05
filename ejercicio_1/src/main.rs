/*
Ejercicio 1 - Introducción
Ítem A

Escribir un programa tal que el hilo main crea un thread hijo que actuará como cliente, mientras 
el padre actúa como servidor. La comunicación se establece para enviar y recibir un saludo, 
por ejemplo: Hola hijo y Buen día Papá.

Ítem B

Modificar el programa del ejercicio anterior para que el servidor pueda gestionar más de un cliente.
*/

use std::thread;
use std::io::prelude::*; //Para poder leer y escribir 
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};


pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap(); //Leo los datos del socket

    let response = "Respuesta del servidor al cliente, deberia printearse en el hostlocal";

    stream.write(response.as_bytes()).unwrap(); // Escribo una espuesta para el cliente
    stream.flush().unwrap();
    //El método flush realiza una espera, previniendo que el programa
    //continúe sin haber escrito en la conexión todos los bytes.

}

fn main() {
    //println!("Hello, world!");
    /*
    let thread = thread::spawn(|| {
       println!("Buen día Papá"); 
    });
    */

    //Server
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // listener listo para aceptar conexiones

    //Server
    let stream = listener.accept(); // Obtengo una conexion establecida de un listener

    match stream {
        Ok((_socket, addr)) => handle_connection(_socket),
        Err(e) => println!("error: {:?}", e),
        }

}

