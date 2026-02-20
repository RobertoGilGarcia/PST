fn search(t: i32, a: &[i32]) -> isize {
    let mut izquierda_a: usize = 0;  // Usamos usize porque los índices en Rust son usize

    // Recorremos el array hasta que encontramos t o llegamos al final
    while izquierda_a < a.len() {
        if a[izquierda_a] == t {  // Comparamos el valor actual con t
            return izquierda_a as isize;  // Si lo encontramos, devolvemos el índice
        }
        izquierda_a += 1;  // Avanzamos al siguiente índice
    }

    -1  // Si no encontramos t, devolvemos -1
}

fn bin_search(t: i32, a: &[i32]) -> isize {
    if a == [] {
        return -1
    }

    let mut izquierda: isize = 0;
    let mut derecha: isize = a.len() as isize -1;
    let  medio: isize = derecha / 2;
    {
        if a[medio as usize] == t  {
            return medio as isize;
        }
        while t != a[medio as usize] && a.len() > 0  && izquierda < derecha{
            if a[izquierda as usize] == t {
                return izquierda as isize;
            }
            izquierda += 1;
            if a[derecha as usize] == t {
                return derecha as isize;
            }
            derecha -= 1;
        }
        -1
    }
}

fn rec_bin_search(t: i32, a: &[i32]) -> isize {
    if a.len() == 0 {
        return -1; // Caso base: arreglo vacío
    }


    let x = a.len() % 2;//puntero al medio
    let mid = if x == 1 { a.len() as isize / 2 } else { a.len() as isize / 2 - 1 };

    //si esta en el medio devuelve la posicion del medio
    if t == a[mid as usize] {
        return mid;
    }
    //si esta en la izquierda, busca en la izquierda
    else if t < a[mid as usize] {
        return rec_bin_search(t, &a[0..mid as usize]);
    }
    //si esta en la derecha, busca en la derecha
    else {
        let result = rec_bin_search(t, &a[(mid + 1) as usize..]);
        if result == -1 {
            return -1; // Si no se encuentra en la derecha, devuelve -1
        } else {
            return mid + 1 + result; // Ajusta el índice si se encuentra en la mitad derecha
        }
    }
}



fn count_forbidden(whitelist: &mut [i32], attempts: &[i32]) -> usize {
    whitelist.sort(); //ordenas la whitelist, por eso es whitelist = &mut
    let mut resultado : usize = 0; //variable que almacena los que no estan en la whitelist
    if attempts == [] { // si no intentan entrar, devuelves 0
        return resultado
    }
    for i in 0..attempts.len() { //recorres todos los intentos uno a uno

        if rec_bin_search(attempts[i], whitelist) == -1 { //buscamos el valor de attempts[i](cada intento) si el resultado de la busqueda es -1 no se ha encontrado
            resultado +=1; // le sumamos uno al resultado de intentos fallidos

        }
    }
    resultado // devolvemos el resultado de intentos fallidos.
}

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};

// Función para leer un fichero de enteros almacenados en path, uno por línea
fn read(path: &str) -> Result<Vec<i32>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        let n = line
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(n);
    }

    Ok(v)
}

#[test]
fn small() {
    let mut whitelist = read("whitelist-small.txt").unwrap();
    let attempts = read("attempts-small.txt").unwrap();

    let whitelist = &mut whitelist[..];
    let attempts = &attempts[..];

    assert_eq!(3, count_forbidden(whitelist, attempts));
}


#[test]
fn large() {
    let mut whitelist = read("whitelist-large.txt").unwrap();
    let attempts = read("attempts-large.txt").unwrap();

    let whitelist = &mut whitelist[..];
    let attempts = &attempts[..];

    assert_eq!(367966, count_forbidden(whitelist, attempts));
}

fn main() {

}
