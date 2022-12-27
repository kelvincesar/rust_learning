struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: T) {
        let mut node = Node::new(value);
        node.next = self.head.take();
        self.head = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }
    fn print(&self) {
        let mut output = String::new();
        let mut current = &self.head;
        while let Some(node) = current {
            output += &format!("{}", node.value);
            output += " -> ";
            current = &node.next;
        }
        output += "NULL";
        println!("{}", output);
    }
    
}



fn main() {
    let mut list = LinkedList::new();
    list.print();

    list.push(1.5);
    list.print();
    list.push(2.5);
    list.print();
    list.push(3.6);
    list.print();

    assert_eq!(list.pop(), Some(3.6));
    list.print();
    assert_eq!(list.pop(), Some(2.5));
    list.print();
    assert_eq!(list.pop(), Some(1.5));
    list.print();
    assert_eq!(list.pop(), None);
    
}
