// VecDeque: Cola doble. Inserción y eliminación eficiente al principio y al final.
// Ideal para estructuras FIFO o acceso a ambos extremos.
// https://doc.rust-lang.org/std/collections/struct.VecDeque.html


/*
- Es un buffer circular (ring-buffer), también denominado Cola de doble extremo (conocida como Double-Ended Queue o Deque):
- Es como la cola que programaste, llamándose a dequeue() como pop_front() y a enqueue() como push_back()
    * Pero también se puede insertar un elemento al principio de la cola con push_front() y extraer el elemento del final con pop_back()
- Puede crecer (se recoloca en el montículo si es necesario)
- También se puede indexar fácilmente cualquier elemento (queue[i])
- Se pueden añadir o extraer fácilmente elementos tanto por el final con push_back(), pop_back(), como por el principio con push_front(), pop_front()
*/


use std::collections::VecDeque;


pub fn run() {
    let mut queue = VecDeque::new();
    
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    println!("primero: {:?}", queue.pop_front());  // Devuelve Some<&V> 
    println!("segundo: {:?}", queue.pop_front());
    println!("tercero: {:?}", queue.pop_front());  
    println!("cuarto : {:?}", queue.pop_front());  // Devuelve None
    // println!("cuarto : {:?}", queue[0]);        // está vacía => panic
 
    println!("queue: {:?}", queue);
}
fn main() {
    todo!();
}
