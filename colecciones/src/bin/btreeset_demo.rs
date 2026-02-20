/*
Sets (conjuntos)
    Los elementos no se repiten: son conjuntos, y sólo tienen la clave, no clave y valor como en un HashMap 
*/
// BTreeSet: Conjunto que mantiene ordenados los elementos.
// https://doc.rust-lang.org/std/collections/struct.BTreeSet.html


use std::collections::BTreeSet;


pub fn run() {
    let mut libros: BTreeSet<&str> = BTreeSet::new();


    libros.insert("Don Quijote de la Mancha");
    libros.insert("La Celestina");
    libros.insert("Lazarillo de Tormes");


    if !libros.contains("Fuenteovejuna") {
        println!(
            "Tenemos {} libros, pero no tenemos Fuenteovejuna",
            libros.len()
        );
    }


    libros.remove("La Celestina");


    for libro in &libros {
        println!("{libro}");
    }
}

fn main() {
    todo!();
}