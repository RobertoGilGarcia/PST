// HashMap: Mapa sin orden. Clave-valor con acceso rápido. Ideal para búsquedas rápidas sin orden.
// https://doc.rust-lang.org/std/collections/struct.HashMap.html


/*
- En Rust se utiliza la colección HashMap<K, V> para almacenar parejas de Valores asociados a Claves
    * En otros lenguajes de programación pueden recibir el nombre de Maps, Diccionarios o Arrays Asociativos
- Las claves han de ser todas del mismo tipo K, y los valores han de ser todos del mismo tipo V
    * Todas las claves han de ser únicas
- En un HashMap podemos insertar un valor para una nueva clave que no está, modificar el valor de una clave que está, consultar el valor de una clave que está, o eliminar una pareja de clave/valor que está.
- Acceder con map[clave] provoca un error en tiempo de ejecución (panic) si no se encuentra la clave, como ocurre cuando en una rodaja o en un array se intenta acceder a una posición que no existe.
- map.get(clave) sin embargo devuelve un Option<&V>
- Los elementos no están ordenados ni por clave K ni por valor V
- Puede crecer (se recolocarán los elementos si no hay espacio)
- Se puede buscar un elemento muy rápidamente con map[i] o map.get(i)
    * Requiere muy pocas comparaciones, por muchos elementos que tenga, muchas menos que con la búsqueda binaria
        + Del orden de 3 a 5 comparaciones, dependiendo de la función hash y del número de elementos
- Muy eficiente para insertar elementos en cualquier posición a diferencia de Vec y VecDeque
    * Si hay colisión, se busca linealmente un hueco
- Pueden elegirse independientemente los tipos de la Clave y del Valor
*/


// API Entry
/*
- Sirve para poder actualizar un valor en caso de que ya exista o que se añada un nuevo valor si no existe
- HashMap ofrece el método entry() que devuelve el enum Entry:


use std::collections::HashMap;


fn subir_la_nota(alumno: &str, acta: &mut HashMap<String, u64>) {
    println!("{:?}", acta.entry(alumno.to_string()));
 
    acta.entry(alumno.to_string())
        .and_modify(|v| *v += 1)   // Si entry() devuelve Occupied, existe la clave alumno y se modifica su nota
                                   // |v| *v += 1 es un cierre: es una función anónima que le pasamos a and_modify() para que la llame, pasándonos
                                   // en el parámetro v el valor asociado a la clave alumno.to_string()
 
        .or_insert(1);             // Si entry() devuelve Vacant, se inserta el valor 1 asociado con la clave alumno.to_string()
}


fn main() {
    let mut acta: HashMap<String, u64> = HashMap::new();
    acta.insert(String::from("Adela"), 7);
 
    subir_la_nota("Adela", &mut acta);
    subir_la_nota("Pepe",  &mut acta);
 
    println!("{:?}", acta);
}
*/


// Obtención de funciones hash para tipos clave de un HashMap en Rust
/*
- La clave en un HashMap tiene que implementar los rasgos (traits) std::hash::Hash y std::hash::Eq
- Los enteros, char y String implementan ambos rasgos
    * También las tuplas, arrays, rodajas y vectores siempre que sus elementos los implementen también
- Los structs y los enums no implementan el rasgo Hash por defecto, pero se puede derivar automáticamente si sus elementos lo implementan también:
#[derive(Clone, PartialEq, Eq, Hash)]
enum MiTipoEnumeradoParaLaClaveDeUnHashMap (
  ...
)


#[derive(Clone, PartialEq, Eq, Hash)]
enum MiTipoStructParaLaClaveDeUnHashMap {
  ...
}
*/


/*
Aplicaciones:
    Aplicación	Clave	Valor
    Mapa de DNS	nombre	dirección IP
    Caché de ARP	dirección IP	dirección Ethernet
    Tabla de switch	dirección Ethernet	Puerto
    Diccionario	palabra	definición
    Índice de libro	capítulo	número de página
    P2P para música	nombre de canción	dirección IP
    Buscador Web	palabra buscada	dirección IP
*/


use std::collections::HashMap;


pub fn run() {
    let mut map: HashMap<&str, u8> = HashMap::new();


    map.insert("Triángulo", 3);
    map.insert("Cuadrado", 4);
    map.insert("Octógono", 8);


    println!("Los triángulos tienen {:?} caras", map.get("Triángulo"));
    println!("Los triángulos {:?} caras", map["Triángulo"]);


    println!("Los pentágonos tienen {:?} caras", map.get("Pentágono"));
    // println!("Los pentágonos tienen {:?} caras", map["Pentágono"]);  // panic!


    // Se puede iterar sobre todos los elementos, 
    // que no están ordenados
    for e in map {
        println!("{e:?}")
    };  
}

fn main() {
    todo!();
}
