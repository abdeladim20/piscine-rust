#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        Self {
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value: value,
            next: self.head.take().map(|node| Box::new(node)),    
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        self.head = self.head.take().and_then(|node| node.next.map(|node| *node));
    }

    pub fn len(&self) -> usize {
        let mut counter = 0 as usize;
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            counter += 1;
            current_node = node.next.as_deref();
        }
        counter
    }
}