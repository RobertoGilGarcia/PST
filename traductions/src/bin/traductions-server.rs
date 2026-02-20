use std::collections::{HashMap};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{env, str};
use std::thread;


    
fn handle_client(mut stream: TcpStream, storage: Arc<Mutex<HashMap<String, String>>>) -> std::io::Result<()> {
    println!("Cliente {} conectado", stream.peer_addr()?);
    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone()?); //buffer del servidor para procesar mensajes de los clientes
    
    let mut message = String::new(); // mensaje que se reciba en modo de String
    while let Ok(bytes_read) = reader.read_line(&mut message) {//control de que no haya problemas en la red
        if bytes_read == 0 {// si me mandan un mensaje con 0 bytes es que ha habido algun problema, se acabo la conexion
            break;
        }


        let words: Vec<&str> = message.trim().split_whitespace().collect(); // transforma el mensaje en un vec de &str ["hola servidor"] -> ["hola", "servidor"]
        match words[0] { //matcheas la primera palabra del vec (el comando que sea .q, .i, .s)
            ".q" => { //desconectarse
                println!("Cliente {} desconectado", stream.peer_addr()?);
                break; //cerrar la conexion con ese cliente.
            }
            ".s" => { //consultar significado de palabra
                let word = words[1]; //coges la palabra despues del .s para consultar su significado
                let unlocked_storage = storage.lock().unwrap(); // te cierras en el candado del arc mutex
                if let Some(traduction) = unlocked_storage.get(word) { //si hay una clave en el HashMap con esa palabra
                    println!("Cliente {} consulta - {}: {}", stream.peer_addr()?, word, traduction); //imprimo lo que hace el cliente
                    writeln!(stream, ".info {}: {}", word, traduction)?; //le mando el .info con la palabra y el significado
                } else { // si no lo hay, imprimo lo que quiere hacer el cliente y le digo que no está esa palabra en el HashMap
                    println!("Cliente {} consulta - {} no está en el diccionario", stream.peer_addr()?, word);
                    writeln!(stream, ".info {} no está en el diccionario", word)?;
                }
            }
            ".i" => { //insertar palabra + significado
                let word = words[1]; // me fijo en la palabra a insertar
                let traduction = message.trim().split_whitespace().skip(2).collect::<Vec<&str>>().join(" "); //meto la definicion en formato vec de %str con separacion entre palabras del " "
                let mut unlocked_storage = storage.lock().unwrap(); // me encierro yo en el arc mutex para cambiarlo
                unlocked_storage.insert(String::from(word), traduction.clone()); //meto la clave -> palabra, definicion -> valor en el HashMap
                println!("Cliente {} inserta - {}: {}", stream.peer_addr()?, word, traduction); //imprimo lo que ha hecho el cliente en el servidor
            }
            _ => {}
        }
        
        message.clear();
    }
    
    
    Ok(())    
}


const NUMBER_OF_ARGS: usize = 2; // 2 ya que es el nombre del archivo y el puerto por el que escucha
 
fn main() -> std::io::Result<()> {
    if env::args().len() != NUMBER_OF_ARGS { // control de que se reciban los argumentos necesarios
        panic!("cargo run --bin dict-server puerto-servidor");
    }
    let server_port: String = env::args().nth(1).unwrap(); // esta es la manera de coger el segundo argumento(el primero es el nombre del fichero) y guardarlo en la variable server_port
 
    let listener = TcpListener::bind(format!("0.0.0.0:{}", server_port))?; //la manera de escuchar cualquier IP en el puerto determinado de la entrada del teclado
    println!("Esperando conexiones...");


    let storage: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new())); //base de datos con palabra -> definicion (HashMap<clave, valor>)
    
    for result_stream in listener.incoming() { // cuando llega un cliente se inicia un thread para él
        let stream = result_stream?;
        let arc_storage: Arc<Mutex<HashMap<String, String>>> = Arc::clone(&storage); // se manda la referencia de la base de datos del server(para poder cambiarla con las definiciones.)


        thread::spawn(move || -> std::io::Result<()> { //spawn del nuevo hilo para cada cliente que se conecte
            handle_client(stream, arc_storage)?;
            Ok(())
        });  
    }
 
    Ok(())
}