/*
Ejercicio 1 - Introducción
Ítem A

Escribir un programa tal que el hilo main crea un thread hijo que actuará como cliente, mientras 
el padre actúa como servidor. La comunicación se establece para enviar y recibir un saludo, 
por ejemplo: Hola hijo y Buen día Papá.

Ítem B

Modificar el programa del ejercicio anterior para que el servidor pueda gestionar 
más de un cliente.
*/
/*
use std::thread;
use std::io::prelude::*; //Para poder leer y escribir 
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};


pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap(); //Leo los datos del socket

    //let response = "Respuesta del servidor al cliente, deberia printearse en el hostlocal";
    let response = "Hola hijo";

    stream.write(response.as_bytes()).unwrap(); // Escribo una espuesta para el cliente
    stream.flush().unwrap();
    //El método flush realiza una espera, previniendo que el programa
    //continúe sin haber escrito en la conexión todos los bytes.

}

fn main() {

    //Server
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Listener failed to bind");
    // listener listo para aceptar conexiones

    //Server
    let stream = listener.accept(); // Obtengo una conexion establecida de un listener

    match stream {
        Ok((_socket, addr)) => handle_connection(_socket),
        Err(e) => println!("error: {:?}", e),
    }


    let thread_hijo = thread::spawn(move || {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878); 

        if let Ok(stream) = TcpStream::connect("127.0.0.1:7878") {
            println!("Buen día Papá");
        } else {
            println!("No se pudo conectar...");
        }
    });

    thread_hijo.join().unwrap();
}

*/





//Como es todo dentro del mismo programa
//voy a pasarme variables de forma tranqui
use std::io::prelude::*;

use std::net::{TcpListener, TcpStream};

use std::sync::mpsc;

use std::io::BufReader;

use std::thread;

fn main() {
    let addres = "0.0.0.0:4560";
    let (sender, handler) = create_thread(addres);
    println!("Termine de preparar el thread");
    set_up_padre(addres, sender);
    if handler.join().is_err() {
        println!("Error cerrando el thread");
    }
}

fn set_up_padre(addres: &str, sender: mpsc::Sender<usize>) {
    let stream_result = TcpListener::bind(addres);
    if sender.send(0).is_err() {
        println!("Error enviando por el sender");
    }
    println!("Le pase clave al thread");
    if let Ok(stream) = stream_result {
        loop {
            let connection_result = stream.accept();
            if let Ok(connection) = connection_result {
                let stream = connection.0;
                let mut reader = BufReader::new(stream);
                let mut string = String::new();
                if let Ok(_) = reader.read_line(&mut string) {
                    println!("Recibido: {}", string);
                }
                let stream_result = TcpStream::connect(connection.1);
                if let Ok(mut stream) = stream_result {
                    if stream.write(&("Hola hijo".as_bytes())).is_err() {
                        println!("Error respondiendole a mi hijo");
                    }
                }
            }
        }
    } else {
        println!("Algo salio mal al abrir el lector");
    }
}

fn create_thread(addres: &str) -> (mpsc::Sender<usize>, thread::JoinHandle<()>) {
    let (sender, reciever) = mpsc::channel();
    let addres_string = String::from(addres);
    let handler = thread::spawn(move || {
        if reciever.recv().is_err() {
            println!("Problema leyendo del channel");
        }
        let stream_result = TcpStream::connect(addres_string);
        if let Ok(mut stream) = stream_result {
            if stream.write(&("Buen dia papa!\n".as_bytes())).is_err() {
                println!("No se pudo enviar el saludo al padre")
            }
            let mut reader = BufReader::new(stream);
            let mut string = String::new();
            if let Ok(_) = reader.read_line(&mut string) {
                println!("Recibido hijo: {}", string);
            }
        }
    });

    (sender, handler)
}