use std::env;
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::str;

const NUMBER_OF_ARGS : usize = 1 + 1; // el 1 + 1 significa que tiene que pasar 2 parametros en el servidor, el binario y el puerto en el que escucha

fn main() -> std::io::Result<()> {
    if env::args().len() != NUMBER_OF_ARGS { // control de que haya 1 + 1 argumentos.
        panic!("cargo run --bin server server_port");
    }

    let server_port = env::args().nth(1).unwrap().parse::<u16>().expect("el puerto debe ser un numero válido");
    // este es el puerto del servidor, se tiene que declarar con esta sintaxis. el nth(1) significa que es la posicion 1 de los argumentos.
    

    let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, server_port))).expect(format!("No puede utilizarse el puerto {}",server_port).as_str());
    println!("Abierto socket en IP: {}, puerto: {}",socket.local_addr()?.ip(), socket.local_addr()?.port());

    loop {
        let mut buffer = [0; 1500];
        socket.recv_from(&mut buffer)?;
        let message = str::from_utf8(&buffer).expect("el mensaje no se puede convertir a string");
        match message.split_once('|') {
            Some((nick, text)) => {
                println!("{},{}", nick, text);
            }
            None => {
                println!("Mensaje erróneo recibido: {}", message);
            }
        }
    }
}