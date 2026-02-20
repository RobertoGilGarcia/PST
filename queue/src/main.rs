#[derive(Debug, Clone)]
struct Queue<T> {
    first: Option<Box<QueueNode<T>>>,
    last: Option<Box<QueueNode<T>>>, 
    size: usize
}


#[derive(Debug, Clone)]
struct QueueNode<T> {
    previous: Option<Box<QueueNode<T>>>,
    value: T,
    next: Option<Box<QueueNode<T>>>
}


impl <T: Eq + Copy> Queue<T> {
    fn new() -> Queue<T> {
        Queue { first: None, last: None, size: 0 }
    }


    fn is_empty(&self) -> bool {
        self.size == 0
    }


    fn enqueue(&mut self, new_value: T) {
        let mut new_node = Box::new(QueueNode {
            previous: None,
            value: new_value,
            next: None
        });
        if let Some(node) = &self.last {
            new_node.previous = Some((*node).clone());
        }
    }
}


fn main() {
}

