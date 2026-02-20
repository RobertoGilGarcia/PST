use std::collections::HashMap;
use std::fs::File;
use std::{env, thread};
use std::io::{prelude::*, BufReader, BufWriter};
use std::net::TcpStream;


use colored::*;


const ARGS: usize = 4;
const CREDENTIALS_FILE_NAME: &str = "credentials.txt";


fn main() -> std::io::Result<()> {
    if env::args().len() != ARGS {
        panic!("cargo run --bin client ip-servidor puerto-servidor nickname");
    }
    let server_ip = env::args().nth(1).unwrap();
    let server_port = env::args().nth(2).unwrap(); 
    let nickname = env::args().nth(3).unwrap();
 
    let mut stream = TcpStream::connect(format!("{}:{}", server_ip, server_port))?;
    
    println!(
        "\nSoy un cliente: {} y he establecido una conexión con el servidor: {}.\n",
        stream.local_addr()?,
        stream.peer_addr()?
    );
    

    let mut credentials: HashMap<String, String> = HashMap::new();
 
    let mut must_send_newuser_message = false;
    match File::open(CREDENTIALS_FILE_NAME) {
        Ok(credentials_file) => {
            let reader = std::io::BufReader::new(credentials_file);
            for line_result in reader.lines() {
                let line = line_result?;
                let columns: Vec<&str> = line.split(' ').collect();
                credentials.insert(columns[0].to_string(), columns[1].to_string());
            }
            if !credentials.contains_key(&nickname) {
                must_send_newuser_message = true;
            }
        }
        Err(_) => {
            must_send_newuser_message = true;
        }
    }
    
    let mut must_interact_with_server = true;
    if must_send_newuser_message {
        writeln!(stream, ".newuser {}", nickname)?;

        let mut message = String::new();
        let mut reader = BufReader::new(stream.try_clone()?); 
        let bytes_read: usize = reader.read_line(&mut message)?; 
        if bytes_read == 0 {
            panic!("Se han recibido 0 bytes como respuesta al mensaje NEWUSER");
        }
        let words: Vec<&str> = message.trim().split_whitespace().collect();
        if words.len() == 3 && words[0] == ".accept" {
            let nickname = words[1];
            let hash = words[2];
            credentials.insert(nickname.to_string(), hash.to_string());
            println!("Conectado al servidor con el nickname '{}'\n", nickname);


            let credentials_file = File::create(CREDENTIALS_FILE_NAME)?;
            let mut writer = BufWriter::new(credentials_file);
            for (nickname, hash) in credentials.clone() {
                writeln!(writer, "{} {}", nickname, hash)?;
            }
            writer.flush()?;
        } else if words.len() == 2 && words[0] == ".reject" {
            let nickname = words[1];
            println!("No se puede hacer utilizar el nickname '{}' porque ya está en uso en el GroupChat", nickname);
            must_interact_with_server = false;
        }
    }


    if must_interact_with_server {
        let hash = credentials.get(&nickname).unwrap();
        let reader = BufReader::new(stream.try_clone()?);
        thread::spawn(|| -> std::io::Result<()> {
            for line_result in reader.lines() {
                let message = line_result?;
                if let Some((".info", text)) = message.split_once(' ') {
                    println!("{}", format!("{}\n", text).red());
                }
            }
            Ok(())
        });


        for line_result in std::io::stdin().lines() {
            let line: String = line_result?; 
            let words: Vec<&str> = line.trim().split_whitespace().collect();
            if let Some(&command) = words.get(0) {
                match command {
                    ".list" => {
                        writeln!(stream, "{} .list", hash)?; 
                    }
                    ".create" => {
                        let group = words[1];
                        writeln!(stream, "{} .create {}", hash, group)?; 
                    }
                    ".join" => {
                        let group = words[1];
                        writeln!(stream, "{} .join {}", hash, group)?;
                    }
                    ".leave" => {
                        writeln!(stream, "{} .leave", hash)?;
                    }
                    ".quit" => {
                        writeln!(stream, "{} .quit", hash)?;
                        break;
                    }
                    _ => {
                        if command.starts_with('.') {
                            println!("{}", format!("Comando desconocido").red());
                        } else {
                            let text = line;
                            writeln!(stream, "{} {}", hash, text)?;
                        }
                    }
                }
                println!();
            }
        }
    }
    
    Ok(())
}
