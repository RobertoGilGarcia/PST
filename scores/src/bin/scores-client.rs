use std::{env, io::{BufReader, prelude::*}, net::TcpStream, thread};

fn main() -> std::io::Result<()> {
    if env::args().len() != 3 {
        panic!("cargo run --bin ranking-client ip-servidor puerto-servidor");
    }

    let server_ip = env::args().nth(1).unwrap();
    let server_port = env::args().nth(2).unwrap();

    let mut stream = TcpStream::connect(format!("{}:{}", server_ip, server_port))?;
    println!("Conectado al servidor. Comandos: .score <usuario> <puntos>, .top, .q");

    let reader = BufReader::new(stream.try_clone()?);
    thread::spawn(|| -> std::io::Result<()> {
        for line in reader.lines() {
            let line = line?;
            if line.starts_with(".info") {
                println!("=== RANKING ===");
            } else if line.contains("→") {
                println!("{}", line);
            }
        }
        Ok(())
    });

    for line in std::io::stdin().lines() {
        let input = line?;
        if input.trim() == ".q" {
            writeln!(stream, ".q")?;
            break;
        }
        writeln!(stream, "{}", input.trim())?;
    }

    Ok(())
}
