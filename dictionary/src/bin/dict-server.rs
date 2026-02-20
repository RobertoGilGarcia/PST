use std::collections::{HashMap};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{env, str};
use std::thread;

struct Storage {
    users: HashMap<String, TcpStream>,
    dictionary: HashMap<String, String>
}
    
fn handle_client(mut stream: TcpStream, storage: Arc<Mutex<Storage>>) -> std::io::Result<()> {
    println!("Cliente {} conectado", stream.peer_addr()?);
    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone()?);
    
    let mut message = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut message) {
        if bytes_read == 0 {
            //println!("Usuario desconectado");
            break;
        }
        //println!("Mensaje recibido: {}", message.trim());

        let words: Vec<&str> = message.trim().split_whitespace().collect();
        match words[0] {
            ".q" => {
                let nickname = words[1];
                println!("Cliente {} desconectado", nickname);
                let mut unlocked_storage = storage.lock().unwrap();
                unlocked_storage.users.remove(nickname);
                for (_, mut user_stream) in &unlocked_storage.users {
                    writeln!(user_stream, ".info El usuario {} se ha desconectado", nickname)?;
                }
                break;
            }
            ".s" => {
                let nickname = words[1];
                let word = words[2];
                let unlocked_storage = storage.lock().unwrap();
                if let Some(definition) = unlocked_storage.dictionary.get(word) {
                    println!("Cliente {} consulta - {}: {}", nickname, word, definition);
                    writeln!(stream, ".info {}: {}", word, definition)?;
                } else {
                    println!("Cliente {} consulta - {} no está en el diccionario", nickname, word);
                    writeln!(stream, ".info {} no está en el diccionario", word)?;
                }
            }
            ".i" => {
                let nickname = words[1];
                let word = words[2];
                let definition = message.trim().split_whitespace().skip(5).collect::<Vec<&str>>().join(" ");
                let mut unlocked_storage = storage.lock().unwrap();
                unlocked_storage.dictionary.insert(String::from(word), definition.clone());
                println!("Cliente {} inserta - {}: {}", nickname, word, definition);
                for (user_nickname, mut user_stream) in &unlocked_storage.users {
                    if nickname != user_nickname {
                        writeln!(user_stream, ".info El usuario {} ha actualizado la definición de la palabra {}: {}", nickname, word, definition)?;
                    }
                }
            }
            ".newuser" => {
                let nickname = words[1];
                print!("Mensaje NEWUSER recibido con el nickname '{}': ", nickname);
                let mut unlocked_storage = storage.lock().unwrap();
                if unlocked_storage.users.contains_key(nickname) {
                    println!("RECHAZADO");
                    writeln!(stream, ".reject {}", nickname)?;
                } else {
                    println!("ACEPTADO");
                    for (_, mut user_stream) in &unlocked_storage.users {
                        writeln!(user_stream, ".info El usuario {} se ha conectado", nickname)?;
                    }
                    unlocked_storage.users.insert(String::from(nickname), stream.try_clone()?);
                    writeln!(stream, ".accept {}", nickname)?;
                }
            }
            _ => {}
        }
        
        message.clear();
    }
    
    
    Ok(())    
}

const NUMBER_OF_ARGS: usize = 1 + 1;
 
fn main() -> std::io::Result<()> {
    if env::args().len() != NUMBER_OF_ARGS {
        panic!("cargo run --bin dict-server puerto-servidor");
    }
    let server_port: String = env::args().nth(1).unwrap(); // Otra alternativa es transformar el String a un u16
 
    let listener = TcpListener::bind(format!("0.0.0.0:{}", server_port))?;
    println!("Esperando conexiones...");

    let storage: Arc<Mutex<Storage>> = Arc::new(Mutex::new(Storage { users: HashMap::new(), dictionary: HashMap::new() }));
    
    for result_stream in listener.incoming() {
        let stream = result_stream?;
        let arc_storage: Arc<Mutex<Storage>> = Arc::clone(&storage);

        thread::spawn(move || -> std::io::Result<()> {
            handle_client(stream, arc_storage)?;
            Ok(())
        });  
    }
 
    Ok(())
}
