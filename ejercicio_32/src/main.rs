fn main() {
    println!("Hello, world!");
}

enum List { //esto es una lista enlazada manual, si esta vacia no sigue, si tiene cosas, esas cosas apuntan a otra lista enlazada.
    Empty,
    NonEmpty(Box<Node>),
}

use List::*;

struct Node { //cada nodo de la lista tiene su valor, y el proximo, al que apunta, que a su vez es una lista.
    value: u8, 
    next: List,
}

impl List {
    fn new() -> List { //constructor de la lista para crear una nueva.
        Empty
    }

    fn exists(&self, value: u8) -> bool { // es para comprobar si existe un elemento en la lista. (recursivamente se busca)
        match self {
            Empty => false,
            NonEmpty(ref node) => { //puntero a un puntero.
                if node.value == value {
                    true
                } else {
                    node.next.exists(value)
                }
            }
        }
    }

    // Ojo: esta implementación es muy ineficiente:
    // añade un elemento al final recorriendo todos los elementos hasta llegar
    // al último:
    fn push(&mut self, new_value: u8) { //añadir por el final un nuevo valor,
        match self {
            Empty => { // si esta vacia, se reemplaza self por un NonEmpty que de valor tiene el que metes y el proximo nodo esta vacio
                *self = NonEmpty(Box::new(Node {
                    value: new_value,
                    next: Empty,
                }));
            }
            NonEmpty(ref mut node) => { // si no esta vacia, el siguiente al que metas pasa a estar vacio, pero de esta forma:
                if let Empty = node.next {  // si el siguiente esta vacio, pasa a estar NonEmpty, con valor, el valor que estes metiendo y el siguiente vacio.
                    node.next = NonEmpty(Box::new(Node {
                        value: new_value,
                        next: Empty,
                    }));
                } else {
                    node.next.push(new_value);// si no esta vacio, vuelves a hacer este proceso recursivamente con el siguiente hasta que encuentres uno de los dos casos de arriba
                }
            }
        }
    }

    fn peek_all_accum(&self, accum: &mut Vec<u8>) { //metodo de una LISTA, depende de como este esa lista, vacia, o no
        match self {
            Empty => {}, // si esta vacia no hay nada que meter al vec
            NonEmpty(node) => { // si no esta vacia, (tiene nodo/s) metes al vec(push) el VALOR del nodo 
                accum.push(node.value);
                node.next.peek_all_accum(accum); //haces lo mismo recursivamente con el siguiente hasta que te encuentres que esta vacio.
            },
        }
    }

    fn peek_all(&self) -> Vec<u8> { // coge los valores de tu lista y los pone en un vec
        let mut vector : Vec<u8> = Vec::new(); // vector declarado en el que se almacena todo 
        self.peek_all_accum(&mut vector); //funcion que acumula en el vector todos los datos.
        vector
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_push_exists() {
        let mut l = super::List::new();
        l.push(3);
        l.push(7);
        l.push(1);
    
        assert_eq!(false, l.exists(25));
        assert_eq!(true, l.exists(1));
    
        assert_eq!(true, l.exists(3));
        assert_eq!(true, l.exists(7));
    
    }

    #[test]
    fn test_peek_all() {
        let mut l = super::List::new();
        l.push(3);
        l.push(7);
        l.push(1);
    
        assert_eq!(vec![3,7,1], l.peek_all());
    }
}