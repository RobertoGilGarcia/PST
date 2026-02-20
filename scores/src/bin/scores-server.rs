use std::collections::{BTreeMap};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{env, thread};

type Ranking = BTreeMap<u32, Vec<String>>;

fn handle_client(mut stream: TcpStream, storage: Arc<Mutex<Ranking>>) -> std::io::Result<()> {
    println!("Cliente {} conectado", stream.peer_addr()?);
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut message = String::new();

    while let Ok(bytes_read) = reader.read_line(&mut message) {
        if bytes_read == 0 {
            break;
        }

        let words: Vec<&str> = message.trim().split_whitespace().collect();
        match words.get(0).map(|s| *s) {
            Some(".q") => {
                println!("Cliente {} desconectado", stream.peer_addr()?);
                break;
            }
            Some(".score") => {
                if words.len() >= 3 {
                    let user = words[1].to_string();
                    if let Ok(score) = words[2].parse::<u32>() {
                        let mut unlocked = storage.lock().unwrap();
                        unlocked.entry(score).or_default().push(user.clone());
                        println!("Cliente {} añade puntuación: {} → {}", stream.peer_addr()?, user, score);
                    }
                }
            }
            Some(".top") => {
                let unlocked = storage.lock().unwrap();
                let mut response = String::new();
                for (score, users) in unlocked.iter().rev() {
                    for user in users {
                        response.push_str(&format!("{} → {}\n", user, score));
                    }
                }
                writeln!(stream, ".info\n{}", response)?;
            }
            _ => {}
        }
        message.clear();
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    if env::args().len() != 2 {
        panic!("cargo run --bin ranking-server puerto-servidor");
    }

    let server_port = env::args().nth(1).unwrap();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", server_port))?;
    println!("Servidor escuchando en el puerto {}", server_port);

    let storage = Arc::new(Mutex::new(BTreeMap::<u32, Vec<String>>::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let storage_clone = Arc::clone(&storage);
        thread::spawn(move || {
            handle_client(stream, storage_clone).unwrap();
        });
    }

    Ok(())
}