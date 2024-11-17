struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    fn size() -> usize {
        self.size
    }

    fn is_empty() -> bool {
        self.size == 0
    }

    fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop_front(&mut self) -> T {
        let old_head = self.head.take();
        match old_head {
            Some(node) => {
                self.head = node.next;
                self.size -= 1;
                node.data
            }
            None => panic!("Cannot pop from an empty list"),
        }
    }

    fn 
}

fn main() {

}