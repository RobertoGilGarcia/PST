// BTreeMap: Mapa ordenado. Mantiene claves ordenadas y tiene eficiencia logarítmica.
// Ideal si necesitas los elementos ordenados por clave.
// https://doc.rust-lang.org/std/collections/struct.BTreeMap.html


/*
- Como un HashMap, pero los elementos están ordenados por la clave.
- Los elementos están ordenados por las claves
    * Ofrece por tanto operaciones como pop_first(), pop_last(), first_key_value(),... que pueden ser útiles para crear colas de prioridades aunque es mejor usar para esto BinaryHeap
- Puede crecer (se recolocan los elementos en el segmento heap si es necesario)
- Se puede acceder a cualquier elemento rápidamente (map[i] o map.get(i))
- Rápido para insertar, aunque no tanto como HashMap
- Se pueden elegir los tipos de Clave y Valor independientemente
- No se pude obtener una rodaja &[T]
- Las claves han de ser todas del mismo tipo, y los valores han de ser del mismo tipo
- Más lento que HashMap por tener que mantener ordenados los elementos
*/


use std::collections::BTreeMap;


pub fn run() {
    let mut map: BTreeMap<&str, i32> = BTreeMap::new();
    
    map.insert("Pentágono", 5);
    map.insert("Triángulo", 3);
    map.insert("Cuadrado", 4);


    println!("Los triángulos tienen {:?} caras", map.get("Triángulo"));
    println!("Los triángulos tienen {:?} caras", map["Triángulo"]);


    // Se puede iterar sobre todos los elementos, 
    // que están ordenados por clave
    for e in map { 
        println!("{e:?}") 
    };  
}
fn main() {
    todo!();
}