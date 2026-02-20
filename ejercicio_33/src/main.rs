fn main() {
    println!("Hello, world!");
}

pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

use BinaryTree::*;

pub struct TreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord + Clone> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        Empty
    }

    pub fn insert(&mut self, new_value: T) {
        match self { //self es el node, depende de lo que encuentre Empty/NonEmpty hace una cosa u otra.
            Empty => {
                *self = NonEmpty(Box::new(TreeNode { // *self es atrvesando el puntero para ver si tiene Empty o NonEmptry
                    value: new_value,
                    left: Empty,
                    right: Empty,
                }))
            }

            NonEmpty(ref mut node) => {
                if new_value <= node.value {
                    node.left.insert(new_value);
                } else {
                    node.right.insert(new_value);
                }
            }
        }
    }

    pub fn exists(&mut self, value: T) -> bool {
        match self {
            Empty => false,

            NonEmpty(ref mut node) => {
                if value == node.value {
                    true
                } else if value < node.value {
                    node.left.exists(value)
                } else {
                    node.right.exists(value)
                }
            }
        }
    }

    pub fn peek_all_accum(&self, accum: &mut Vec<T>, ascending: bool) { //funcion que NO DEVUELVE NADA solo acumula los valores en un vec
        match self {
            Empty => {} // si esta vacio mi Btree no hay nada para sacar
            NonEmpty(node) => { // si hay algo en mi nodo
                if ascending { // si es ascendente, tengo que coger el de la izquierda  primero porque es mas pequeño
                    node.left.peek_all_accum(accum, ascending);
                    accum.push(node.value.clone()); // guarda el valor izquierdo
                    node.right.peek_all_accum(accum, ascending); // mira el derecho
                } else {
                    node.right.peek_all_accum(accum, ascending); // si es descendente, tengo que coger el de la derecha porque es el mas grande
                    accum.push(node.value.clone()); // guardo el valor derecho 
                    node.left.peek_all_accum(accum, ascending); // miro el izquierdo.
                } // en estos dos esta el metodo .clone(), que se necesita para no perder el valor original del arbol, solo queremos en realidad sacar un vec con orden
                // asdencente o descendente, no queremos vaciar el árbol. Para vaciarlo deberia ser una referencia mutable EXCLUSIVA para evitar mutex y quitar el .clone()
            }
        }
    }


    pub fn peek_all(&self, ascending: bool) -> Vec<T> { // es el mismo metodo de antes, declara un vector y mete todos los valores ordenados(ascendente/descendente) y devuelve el vector.
        let mut vector: Vec<T> = Vec::new();
        self.peek_all_accum(&mut vector, ascending);
        vector
    }


}

#[cfg(test)]
mod tests {

    #[test]
    fn test_insert_exists() {
        let mut tree = super::BinaryTree::new();

        tree.insert("Putin");
        tree.insert("Jinping");
        tree.insert("Feijoo");
        tree.insert("Sánchez");
        tree.insert("Trump");

        assert_eq!(true, tree.exists("Trump"));
        assert_eq!(false, tree.exists("Clinton"));
    }

    #[test]
    fn peek_all_ascending() {
        let mut tree = super::BinaryTree::new();

        tree.insert("Putin");
        tree.insert("Jinping");
        tree.insert("Feijoo");
        tree.insert("Sánchez");
        tree.insert("Trump");

        assert_eq!(
            vec!["Feijoo", "Jinping", "Putin", "Sánchez", "Trump"],
            tree.peek_all(true)
        );
    }

    #[test]
    fn peek_all_descending() {
        let mut tree = super::BinaryTree::new();

        tree.insert("Putin");
        tree.insert("Jinping");
        tree.insert("Feijoo");
        tree.insert("Sánchez");
        tree.insert("Trump");

        assert_eq!(
            vec!["Trump", "Sánchez", "Putin", "Jinping", "Feijoo"],
            tree.peek_all(false)
        );
    }
}