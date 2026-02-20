use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{env, str};
use std::thread;
use rand::Rng;


#[derive(Debug)]
struct User {
    nickname: String,
    group: Option<String>,
}


struct Storage {
    nicknames: HashSet<String>,
    users: HashMap<String, User>,
    groups: HashMap<String, HashMap<String, TcpStream>>
}

fn hash_nickname(nickname: &str) -> u64 {
    let random_number= rand::thread_rng().gen_range(0..=u32::MAX);
    let input: String = format!("{}{}", nickname, random_number);
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn handle_list(stream: &mut TcpStream, hash: &str, storage: &Arc<Mutex<Storage>>) -> std::io::Result<()> {
    let unlocked_storage = storage.lock().unwrap();
    let nickname = &unlocked_storage.users.get(hash).unwrap().nickname;
    println!("Mensaje LIST recibido del usuario '{}'", nickname);
    if unlocked_storage.groups.is_empty() {
        writeln!(stream, ".info No existe ningún grupo actualmente")?;
    } else {
        let group_names: Vec<String> = unlocked_storage.groups.keys().cloned().collect();
        writeln!(stream, ".info Lista de grupos: {}", group_names.join(" "))?;
    }
    Ok(())
}

fn handle_leave(stream: &mut TcpStream, hash: &str, storage: &Arc<Mutex<Storage>>) -> std::io::Result<()> {
    let mut unlocked_storage = storage.lock().unwrap();
    let user = unlocked_storage.users.get(hash).unwrap();
    let nickname = user.nickname.clone();
    let group = user.group.clone();
    print!("Mensaje LEAVE recibido del usuario '{}': ", nickname);
    match group {
        Some(group_name) => {
            let members = unlocked_storage.groups.get_mut(&group_name).unwrap();
            for (member_hash, mut stream) in members.iter() {
                if member_hash != hash {
                    writeln!(stream, ".info {} ha salido del grupo", nickname)?;
                }
            }
            members.remove(hash);
            let user = unlocked_storage.users.get_mut(hash).unwrap();
            user.group = None;
            println!("Grupo '{}': {:?}", group_name, unlocked_storage.groups);
            println!("ha abandonado el grupo {} y se ha informado al resto de integrantes del mismo", group_name);
            writeln!(stream, ".info Te has salido del grupo '{}'", group_name)?;
        }
        None => {
            println!("no pertenece a ningún grupo");
            writeln!(stream, ".info Todavía no estás en ningún grupo")?;
        }
    }
    Ok(()) 
}

fn handle_create(stream: &mut TcpStream, hash: &str, group_name: &str, storage: &Arc<Mutex<Storage>>) -> std::io::Result<()> {
    let mut unlocked_storage = storage.lock().unwrap();
    let user = unlocked_storage.users.get(hash).unwrap();
    let nickname = user.nickname.clone();
    print!("Mensaje CREATE recibido del usuario '{}' con el nombre de grupo '{}': ", nickname, group_name);
    if unlocked_storage.groups.contains_key(group_name) {
        println!("ya existe un grupo con el mismo nombre");
        writeln!(stream, ".info No puedes crear el grupo '{}' porque ya existe uno con el mismo nombre", group_name)?;
    } else {
        unlocked_storage.groups.insert(group_name.into(), HashMap::new());
        println!("el grupo ha sido creado");
        writeln!(stream, ".info Has creado el grupo '{}'", group_name)?;
    }
    Ok(())
}

fn handle_join(stream: &mut TcpStream, hash: &str, group_name: &str, storage: &Arc<Mutex<Storage>>) -> std::io::Result<()> {
    let mut unlocked_storage = storage.lock().unwrap();
    let user = unlocked_storage.users.get(hash).unwrap();
    let nickname = user.nickname.clone();
    let group = user.group.clone();
    println!("Mensaje JOIN recibido del usuario '{}' con el nombre de grupo '{}': ", nickname, group_name);
    match group {
        Some(user_group_name) => {
            println!("el usuario {} ya pertenece a otro grupo, concretamente al grupo '{}'", nickname, user_group_name);
            writeln!(stream, ".info No puedes unirte al grupo '{}'. Primero tienes que salirte de tu grupo actual", group_name)?;
        }
        None => {
            if let Some(members) = unlocked_storage.groups.get_mut(group_name) {
                for (_, member_stream) in members.iter_mut() {
                    writeln!(member_stream, ".info {} se ha unido al grupo", nickname)?;
                }
                members.insert(hash.to_string(), stream.try_clone()?);
                let user = unlocked_storage.users.get_mut(hash).unwrap();
                user.group = Some(group_name.to_string());
                println!("se ha unido a él el usuario {}", nickname);
                writeln!(stream, ".info Te has unido al grupo '{}'", group_name)?;
                println!("Grupo '{}': {:?}", group_name, unlocked_storage.groups);
            } else {
                println!("no existe un grupo con ese nombre");
                writeln!(stream, ".info No puedes unirte al grupo '{}' porque no existe", group_name)?;
            }
        }
    }
    Ok(())
}

fn handle_text(stream: &mut TcpStream, hash: &str, text: &str, storage: &Arc<Mutex<Storage>>) -> std::io::Result<()> {
    let unlocked_storage = storage.lock().unwrap();
    let user = unlocked_storage.users.get(hash).unwrap();
    let nickname = &user.nickname;
    let group = user.group.clone();
    println!("Mensaje TEXT recibido del usuario '{}' con el texto: {}", nickname, text);
    match group {
        Some(group_name) => {
            let members = unlocked_storage.groups.get(&group_name).unwrap();
            println!("Enviando el mensaje a todos los miembros del grupo '{}' excepto a '{}'", group_name, nickname);
            for (member_hash, mut stream) in members.iter() {
                if member_hash != hash {
                    writeln!(stream, ".info {}: {}", nickname, text)?;
                    let member_nickname = &unlocked_storage.users.get(member_hash).unwrap().nickname;
                    println!("* Mensaje enviado a '{}'", member_nickname);
                }
            }
        }
        None => {
            writeln!(stream, ".info Todavía no estás en ningún grupo")?;
        }
    }
    Ok(())
}

fn handle_quit(stream: &mut TcpStream, hash: &str, storage: &Arc<Mutex<Storage>>) -> std::io::Result<()> {
    let mut unlocked_storage = storage.lock().unwrap();
    let user = unlocked_storage.users.get(hash).unwrap();
    let nickname = user.nickname.clone();
    let group = user.group.clone();
    println!("Mensaje QUIT recibido del usuario '{}'", nickname);
    match group {
        Some(group_name) => {
            let members = unlocked_storage.groups.get_mut(&group_name).unwrap();
            for (member_hash, mut stream) in members.iter() {
                if member_hash != hash {
                    writeln!(stream, ".info {} se ha desconectado del chat", nickname)?;
                }
            }
            members.remove(hash);
            let user = unlocked_storage.users.get_mut(hash).unwrap();
            user.group = None;
            println!("Grupo '{}': {:?}", group_name, unlocked_storage.groups);
            println!("ha abandonado el grupo {}, avisando al resto de integrantes, y se ha desconectado del chat", group_name);
        }
        None => {}
    }
    Ok(())
}
    

fn handle_client(mut stream: TcpStream, storage: Arc<Mutex<Storage>>) -> std::io::Result<()> {
    println!(
        "\nSoy el servidor: {} y he recibido una conexión entrante de un cliente: {}.\n",
        stream.local_addr()?,
        stream.peer_addr()?,
    );

    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone()?);
    let mut message = String::new();

    while let Ok(bytes_read) = reader.read_line(&mut message) {
        if bytes_read == 0 {
            break;
        }

        let words: Vec<&str> = message.trim().split_whitespace().collect();

        if words.len() == 2 && words[0] == ".newuser" {
            let nickname = words[1];
            print!("Mensaje NEWUSER recibido con el nickname '{}': ", nickname);
            {
                let mut unlocked_storage = storage.lock().unwrap();
                if unlocked_storage.nicknames.contains(nickname) {
                    println!("RECHAZADO");
                    writeln!(stream, ".reject {}", nickname)?;
                } else {
                    println!("ACEPTADO");
                    unlocked_storage.nicknames.insert(nickname.to_string());
                    let hash = hash_nickname(nickname).to_string(); 
                    unlocked_storage.users.insert(hash.clone(), User {nickname: nickname.to_string(), group: None});
                    writeln!(stream, ".accept {} {}", nickname, hash.clone())?;
                    println!("Nombres de usuario: {:?}", unlocked_storage.nicknames);
                    println!("Usuarios: {:?}", unlocked_storage.users);
                }
            }
        } else if words.len() >= 2 {
            let hash = words[0];

            {
                let unlocked_storage = storage.lock().unwrap();
                if !unlocked_storage.users.contains_key(hash) {
                    message.clear();
                    continue;
                }
            }

            let command = words[1];

            match (command, words.len()) {
                (".list", 2) => handle_list(&mut stream, hash, &storage)?,
                (".create", 3) => handle_create(&mut stream, hash, words[2], &storage)?,
                (".join", 3) => handle_join(&mut stream, hash, words[2], &storage)?,
                (".leave", 2) => handle_leave(&mut stream, hash, &storage)?,
                (".quit", 2) => {
                    handle_quit(&mut stream, hash, &storage)?;
                    break;
                }

                _ => {
                    if let Some((_, text)) = message.trim().split_once(' ') {
                        handle_text(&mut stream, hash, text, &storage)?;
                    } else {
                        writeln!(stream, ".info Comando no reconocido")?;
                    }
                }
            }
        }

        message.clear();
    }

    Ok(())
}




const ARGS: usize = 2;
 
fn main() -> std::io::Result<()> {
    if env::args().len() != ARGS {
        panic!("cargo run --bin server puerto-servidor");
    }
    let server_port = env::args().nth(1).unwrap();
 
    let listener = TcpListener::bind(format!("0.0.0.0:{}", server_port))?;
    println!("Esperando conexiones de clientes...");


    let storage: Arc<Mutex<Storage>> = Arc::new(Mutex::new(Storage { 
        nicknames: HashSet::new(), 
        users: HashMap::new(), 
        groups: HashMap::new()
    }));
    
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
