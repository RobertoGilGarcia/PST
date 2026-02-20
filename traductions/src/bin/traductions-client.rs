use std::{env, thread};
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;


const NUMBER_OF_ARGS: usize = 3; // controlo que los argumentos por teclado sean nombre fichero, IP server, puerto server


fn main() -> std::io::Result<()> {
    if env::args().len() != NUMBER_OF_ARGS {// control de que me pasen los argumentos necesarios
        panic!("cargo run --bin dict-client ip-servidor puerto-servidor");
    }
    let server_ip = env::args().nth(1).unwrap(); // guardo la IP del server en una variable
    let server_port = env::args().nth(2).unwrap(); // guardo el puerto del server en una variable
 
    let mut stream = TcpStream::connect(format!("{}:{}", server_ip, server_port))?; //me conecto al servidor(se genera el stream con TCP)
    
    println!("Conectado al servidor de diccionario. Introduce comandos (\".q\" para salir):");


    let reader = BufReader::new(stream.try_clone()?); //genero el buffer para el cliente
    thread::spawn(|| -> std::io::Result<()> {
        for line_result in reader.lines() {
            let message = line_result?;
            if let Some((".info", text)) = message.split_once(' ') {
                println!("{}", format!("{}", text));
            }
        }
        Ok(())
    });


    for line_result in std::io::stdin().lines() { // para cada linea del buffer
        let line: String = line_result?; //guardo un String que es toda la linea que se mande
        let words: Vec<&str> = line.trim().split_whitespace().collect(); //cambio la linea a formato vec de &str ["hola servidor"] -> ["hola", "servidor"]
        if let Some(&command) = words.get(0) { // si me devuelve un Ok()
            match command { //matcheo lo que quiero hacer depende del comando
                ".q" => { 
                    writeln!(stream, ".q")?; // le mando el .q al server y acabo
                    break;
                }
                ".s" => { //consultar significado de palabra
                    let word = words[1]; // me quedo con la segunda palabra del vec (la palabra)
                    writeln!(stream, ".s {}", word)?; // le mando la palabra de la cual quiero saber el significado
                }
                ".i" => { // insertar palabra + significado
                    /* 
                    OPCION 1 -> bucle for para recorrer el vec
                    let mut text = String::new();
                    for word in words {
                        text.push_str(word);
                        text.push_str(" ");
                    }
                    writeln!(stream, "{}", text)?;
                    */
                    writeln!(stream, "{}", line.trim())?; //OPCION 2 -> devolver la linea entera, que ya tiene comando + palabra + significado
                }
                _ => { //control de cualquier otra cosa
                    println!("comando no reconocido");
                }
            }
        }
    }
    
    Ok(())
}
