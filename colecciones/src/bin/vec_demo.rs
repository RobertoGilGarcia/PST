// Vec: Vector dinámico. Almacena elementos en una lista contigua en memoria.
// Acceso aleatorio eficiente (por índice). Crece dinámicamente.
// Ideal para listas ordenadas de elementos donde importa el orden y se requiere acceso rápido.
// https://doc.rust-lang.org/std/vec/struct.Vec.html


/*
- Los arrays requieren que su tamaño sea conocido en tiempo de compilación para poder ajustar la pila a su tamaño
- Si no se sabe cuánto espacio se necesitará se utiliza un vector, de tipo Vec<T>, que es un tipo parecido a los arrays, pero que puede crecer en tiempo de ejecución
- El Vec (puntero, capacidad y longitud) se almacena en la pila, pero los valores de sus elementos se almacenan en el montículo (heap)
    * El puntero es la dirección en el montículo donde están sus valores
- Se puede indexar como un array, y se pueden obtener rodajas
- push() para añadir elemento al final y pop() para extraer el último elemento añadido (política LIFO)
- Se puede indexar cualquier elemento rápidamente: vector[i]
- Se pueden añadir (push) y extraer (pop) elementos muy rápidamente, pero sólo al final (política LIFO, como la pila que hiciste en tu ejercicio)
- El vector / Vec NO es una referencia
    * vector es el propietario de los valores que están en el montículo: cuando el ámbito de vector termina se libera la memoria ocupada en el montículo por sus valores
- Cuando se pasa como parámetro por referencia, sólo se copia la dirección de memoria en la que está el valor del Vec: (puntero, capacidad y longitud)
*/


pub fn run() {
    let mut vector = Vec::new(); // Creamos vector vacío inicialmente
    
    vector.push(10);  // Añadimos 3 elementos
    vector.push(11);
    vector.push(12);


    println!("{}", vector[1]);  // Acceso por índice


    while let Some(e) = vector.pop() {  // Extracción LIFO
        println!("{e}");
    }
    println!("{:?}", vector);
}

fn main() {
    todo!();
}