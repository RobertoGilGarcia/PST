pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}
pub struct TreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}


use BinaryTree::*;


impl<T: Ord + Clone> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        Empty
    }


    pub fn peek_all_accum(&self, accum: &mut Vec<T>, ascending: bool) { //auxiliar para recopilar recursivamente
        match self {
            Empty => {}
            NonEmpty(node) => {
                if ascending {
                    node.left.peek_all_accum(accum, ascending);
                    accum.push(node.value.clone());
                    node.right.peek_all_accum(accum, ascending);
                } else {
                    node.right.peek_all_accum(accum, ascending);
                    accum.push(node.value.clone());
                    node.left.peek_all_accum(accum, ascending);
                }
            }
        }
    }


    pub fn peek_all(&self, ascending: bool) -> Vec<T> { //te devuelve todos los elementos del arbol en un vector
        let mut accum: Vec<T> = Vec::new();
        self.peek_all_accum(&mut accum, ascending);
        accum
    }


    pub fn insert(&mut self, new_value: T) { //insertar valor en el arbol
        match self {
            Empty => {
                *self = NonEmpty(Box::new(TreeNode {
                    value: new_value,
                    left: Empty,
                    right: Empty,
                }))
            }
            NonEmpty(node) => {
                if new_value <= node.value {
                    node.left.insert(new_value);
                } else {
                    node.right.insert(new_value);
                }
            }
        }
    }


    pub fn exists(&self, value: T) -> bool { //comprobar si un valor está en el arbol
        match self {
            Empty => {
                false
            }
            NonEmpty(node) => {
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


    fn number_of_nodes(&self) -> usize { //el numero de nodos que tiene el arbol
        match self {
            Empty => {
                0
            }
            NonEmpty(node) => {
                node.left.number_of_nodes() + 1 + node.right.number_of_nodes()
            }
        }
    }


    fn depth(&self) -> usize { //profundidad del arbol
        match self {
            Empty => {
                0
            }
            NonEmpty(node) => {
                1 + node.left.depth().max(node.right.depth())
            }
        }
    }


    fn number_of_branches_v1(&self) -> usize { //numero de ramas del arbol con auxiliar
        if let Empty = self {
            0
        } else {
            self.number_of_nodes() - 1
        }
    }


    fn number_of_branches_v2(&self) -> usize { //numero de ramas sin auxiliar
        match self {
            Empty => {
                0
            }
            NonEmpty(node) => {
                let mut result: usize = 0;
                if let NonEmpty(_left_son) = &node.left {
                    result += 1;
                }
                if let NonEmpty(_right_son) = &node.right {
                    result += 1;
                }
                result + node.left.number_of_branches_v2() + node.left.number_of_branches_v2()
            }
        }
    }


    fn number_of_leaves(&self) -> usize { // numero de nodos hojas
        match self {
            Empty => {
                0
            }
            NonEmpty(node) => {
                match (&node.left, &node.right) {
                    (Empty, Empty) => {
                        1
                    }
                    _ => {
                        node.left.number_of_leaves() + node.right.number_of_leaves()
                    }
                }
            }
        }
    }


    fn min(&self) -> Option<T> { // el nodo con el valor mas pequeño recursivo
        match self {
            Empty => {
                None
            }
            NonEmpty(node) => {
                if let Empty = &node.left {
                    Some(node.value.clone())
                } else {
                    node.left.min()
                }
            }
        }
    }


    fn min_nr(&self) -> Option<T> { // el minimo valor del arbol iterativo
        let mut result = None;
        
        let mut current = self;
        while let NonEmpty(node) = current {
            result = Some(node.value.clone());
            current = &node.left;
        }


        return result;
    }


    fn max(&self) -> Option<T> { //el maximo valor recursivo
        match self {
            Empty => {
                None
            }
            NonEmpty(node) => {
                if let Empty = &node.right {
                    Some(node.value.clone())
                } else {
                    node.right.max()
                }
            }
        }
    }


    fn max_nr(&self) -> Option<T> { //el maximo valor iterativo
        let mut result = None;
        
        let mut current = self;
        while let NonEmpty(node) = current {
            result = Some(node.value.clone());
            current = &node.right;
        }


        return result;
    }


    fn leaves_aux(&self, accum: &mut Vec<T>) { //numero de hojas auxiliar recursivo
        match self {
            Empty => {}
            NonEmpty(node) => {
                match (&node.left, &node.right) {
                    (Empty, Empty) => {
                        accum.push(node.value.clone());
                    }
                    _ => {
                        node.left.leaves_aux(accum);
                        node.right.leaves_aux(accum);
                    }
                }
            }
        }
    }


    fn leaves(&self) -> Vec<T> { //numero de hojas 
        let mut accum: Vec<T> = Vec::new();
        self.leaves_aux(&mut accum);
        return accum;
    }


    fn preorder_aux(&self, accum: &mut Vec<T>) { // auxiliar para preorden
        match self {
            Empty => {}
            NonEmpty(node) => {
                accum.push(node.value.clone());
                node.left.preorder_aux(accum);
                node.right.preorder_aux(accum);
            }
        }
    }


    fn preorder(&self) -> Vec<T> { //arbol en un vec con orden en preorden
        let mut accum: Vec<T> = Vec::new();
        self.preorder_aux(&mut accum);
        return accum;
    }


    fn inorder_aux(&self, accum: &mut Vec<T>) { //inorden auxiliar recursivo
        match self {
            Empty => {}
            NonEmpty(node) => {
                node.left.inorder_aux(accum);
                accum.push(node.value.clone());
                node.right.inorder_aux(accum);
            }
        }
    }


    fn inorder(&self) -> Vec<T> { //arbol en un vec con orden en inorden
        let mut accum: Vec<T> = Vec::new();
        self.inorder_aux(&mut accum);
        return accum;
    }


    fn posorder_aux(&self, accum: &mut Vec<T>) { //auxiliar posorden recursivo
        match self {
            Empty => {}
            NonEmpty(node) => {
                node.left.posorder_aux(accum);
                node.right.posorder_aux(accum);
                accum.push(node.value.clone());
            }
        }
    }


    fn posorder(&self) -> Vec<T> { //arbol en un vec con orden en posorden
        let mut accum: Vec<T> = Vec::new();
        self.posorder_aux(&mut accum);
        return accum;
    }
}


fn main() {
    let mut tree = BinaryTree::new();


    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(2);
    tree.insert(7);
    tree.insert(12);
    tree.insert(17);


    println!("Tree in ascending order: {:?}", tree.peek_all(true));
    println!("Tree in descending order: {:?}", tree.peek_all(false));


    println!("Does 7 exist?: {}", tree.exists(7));
    println!("Does 12 exist?: {}", tree.exists(12));
    println!("Does 20 exist?: {}", tree.exists(20));


    println!("Number of nodes: {}", tree.number_of_nodes());


    println!("Depth of tree: {}", tree.depth());


    println!("Number of branches (v1): {}", tree.number_of_branches_v1());
    println!("Number of branches (v2): {}", tree.number_of_branches_v2());


    println!("Number of leaves: {}", tree.number_of_leaves());


    println!("Minimum value in tree: {:?}", tree.min());
    println!("Minimum value in tree: {:?}", tree.min_nr());
    println!("Maximum value in tree: {:?}", tree.max());
    println!("Maximum value in tree: {:?}", tree.max_nr());


    println!("Leaves: {:?}", tree.leaves());
    println!("Preorder: {:?}", tree.preorder());
    println!("Inorder: {:?}", tree.inorder());
    println!("Posorder: {:?}", tree.posorder());


    let empty_tree: BinaryTree<i32> = BinaryTree::new();
    println!("Empty tree number of nodes: {}", empty_tree.number_of_nodes());
    println!("Empty tree depth: {}", empty_tree.depth());
    println!("Empty tree min: {:?}", empty_tree.min());
    println!("Empty tree max: {:?}", empty_tree.max());
    println!("Empty tree leaves: {}", empty_tree.number_of_leaves());
}

