enum List {
    Empty,
    NonEmpty(Box<Node>),
}
use List::*;
struct Node {
    value: u8,
    next: List,
}


impl List {
    fn new() -> List {
        Empty
    }


    fn peek_all_accum_nr(&self, accum: &mut Vec<u8>) {
        let mut current = self;
        while let NonEmpty(node) = current {
            accum.push(node.value);
            current = &node.next;
        }
    }


    fn peek_all_accum(&self, accum: &mut Vec<u8>) {
        match self {
            NonEmpty(node) => {
                accum.push(node.value);
                node.next.peek_all_accum(accum);
            }
            _ => {}
        }
    }


    fn peek_all(&self) -> Vec<u8> {
        let mut accum: Vec<u8> = Vec::new();
        self.peek_all_accum(&mut accum);
        accum
    }
    
    fn exists(&self, value: u8) -> bool {
        match self {
            Empty => false,
            NonEmpty(node) => {
                if node.value == value {
                    true
                } else {
                    node.next.exists(value)
                }
            }
        }
    }


    fn exists_nr(&self, value: u8) -> bool {
        let mut current = self; // Empezamos en la raíz de la lista
        while let NonEmpty(node) = current {
            if node.value == value {
                return true; // Si encontramos el valor, devolvemos `true`
            }
            current = &node.next; // Nos movemos al siguiente nodo
        }
        false
    }


    fn push(&mut self, new_value: u8) {
        match self {
            Empty => {
                *self = NonEmpty(Box::new(Node {
                    value: new_value,
                    next: Empty,
                }));
            }
            NonEmpty(node) => {
                if let Empty = node.next {
                    node.next = NonEmpty(Box::new(Node {
                        value: new_value,
                        next: Empty,
                    }));
                } else {
                    node.next.push(new_value);
                }
            }
        }
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


        assert_eq!(vec![3, 7, 1], l.peek_all());
    }




    #[test]
    fn test_peek_all_accum_nr() {
        let mut l = super::List::new();
        l.push(3);
        l.push(7);
        l.push(1);


        let mut all = Vec::new();
        l.peek_all_accum_nr(&mut all);
        assert_eq!(vec![3, 7, 1], all);
    }


    #[test]
    fn test_push_exists_nr() {
        let mut l = super::List::new();
        l.push(3);
        l.push(7);
        l.push(1);


        assert_eq!(false, l.exists_nr(25));
        assert_eq!(true, l.exists_nr(1));


        assert_eq!(true, l.exists_nr(3));
        assert_eq!(true, l.exists_nr(7));


        assert_eq!(false, l.exists_nr(42));
    }
    
}
fn main() {
    todo!();
}
