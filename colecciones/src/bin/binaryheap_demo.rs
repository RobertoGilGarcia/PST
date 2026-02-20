// BinaryHeap: Cola de prioridad (max-heap por defecto).
// Ideal para extraer el mayor (o menor, invirtiendo el orden) rápidamente.
// https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
/*
- Permite devolver el valor más grande almacenado muy rápidamente
    * Mejor que BTreeSet para implementar una cola de prioridades
- Los elementos están "ligeramente ordenados", y se pueden repetir, a diferencia de BTreeSet, que mantiene ordenados los elementos pero no se pueden repetir
- Si se extrae el mayor, se recolocan los elementos restantes con poco coste y se puede volver a obtener rápidamente el mayor de ellos
- Insertar un nuevo elemento también tiene poco coste
- Puede haber varios elementos iguales, mientras que en BTreeSet, aunque están ordenados, no puede haber elementos iguales.
- Útil para implementar colas de prioridades
    * Cada elemento tiene una prioridad y siempre queremos obtener el de mayor prioridad (el mayor)
- Tienen que poderse comparar los elementos: tienen que implementar el método cmp() del trait std::Ord y el método partial_cmp() del trait std::PartialOrd
    * La mayoría de los tipos implementan estos traits, pero podemos necesitar definir un orden particular como mostramos en el siguiente ejemplo
*/


// Implementación de los traits std::Ord y std::PartialOrd
/*
- El método cmp() del trait std::Ord devuelve un valor del enum Ordering
- El método partial_cmp() del trait std::PartialOrd devuelve un valor de Option<Ordering> ya que define una relación de orden parcial: no todos los elementos se pueden comparar, devolviendo en este caso None
*/


use std::collections::BinaryHeap;
use std::cmp::Ordering;


#[derive(Debug, Eq, PartialEq)] 
struct Persona {
    nombre: String,
    edad  : u32,
}


// Implementamos el trait Ord para poder comparar según la edad
impl Ord for Persona {
    fn cmp(&self, other: &Self) -> Ordering {
        self.edad.cmp(&other.edad)
    }
}


// Implementamos el trait PartialOrd para poder comparar según la edad
// llamando a cmp() que acabamos de implementar: el orden parcial es el orden total
impl PartialOrd for Persona {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/*
- Cola de prioridad según el orden marcado por la edad de las personas
- Aunque almacena además el nombre, ordena sólo por edad
*/


pub fn run() {
    let mut queue = BinaryHeap::new();


    queue.push(Persona { nombre: String::from("Juan"), edad: 30 });
    queue.push(Persona { nombre: String::from("Alejandra"), edad: 25 });
    queue.push(Persona { nombre: String::from("Ludmila"), edad: 35 });


    // Obtenemos los elementos por orden de prioridad: el de mayor edad primero
    while let Some(persona) = queue.pop() {
        println!("{:?} con edad {}", persona.nombre, persona.edad);
    }
}
fn main() {
    todo!();
}