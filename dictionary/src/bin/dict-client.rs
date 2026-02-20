use std::{env, thread};
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

const NUMBER_OF_ARGS: usize = 1 + 3;

fn main() -> std::io::Result<()> {
    if env::args().len() != NUMBER_OF_ARGS {
        panic!("cargo run --bin dict-client ip-servidor puerto-servidor nickname");
    }
    let server_ip = env::args().nth(1).unwrap();
    let server_port = env::args().nth(2).unwrap(); // Otra alternativa es transformar el String a un u16
    let nickname = env::args().nth(3).unwrap();

    let mut stream = TcpStream::connect(format!("{}:{}", server_ip, server_port))?;
    
    writeln!(stream, ".newuser {}", nickname)?;
    let mut message = String::new();
    let mut reader = BufReader::new(stream.try_clone()?);
    let bytes_read: usize = reader.read_line(&mut message)?;
    if bytes_read == 0 {
        panic!("Se han recibido 0 bytes como respuesta del servidor al mensaje NEWUSER");
    }
    let words: Vec<&str> = message.trim().split_whitespace().collect();
    match words[0] {
        ".accept" => {
            let nickname = words[1];
            println!("Conectado al servidor con el nickname '{}'", nickname);
        }
        ".reject" => {
            let nickname = words[1];
            println!("No se puede hacer utilizar el nickname '{}' porque ya está en uso", nickname);
            return Ok(());
        }
        _ => {}
    }
   
    let reader = BufReader::new(stream.try_clone()?);
    thread::spawn(|| -> std::io::Result<()> {
        for line_result in reader.lines() {
            let message = line_result?;
            if let Some((".info", text)) = message.split_once(' ') {
                println!("{}", format!("{}", text));
            }
        }
        Ok(())
    });

    println!("Introduce comandos (\".q\" para salir):");
    for line_result in std::io::stdin().lines() {
        let line: String = line_result?; 
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        if let Some(&command) = words.get(0) {
            match command {
                ".q" => {
                    writeln!(stream, ".q {}", nickname)?;
                    break;
                }
                ".s" => {
                    let word = words[1];
                    writeln!(stream, ".s {} {}", nickname, word)?;
                }
                ".i" => {
                    let word = words[1];
                    let mut text = String::new();
                    for word in words {
                        text.push_str(word);
                        text.push_str(" ");
                    }
                    writeln!(stream, ".i {} {} {}", nickname, word, text)?;
                }
                _ => {
                    println!("comando no reconocido");
                }
            }
        }
    }
    
    Ok(())
}
