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
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        let old_head = self.head.take();
        let next = old_head.map(Box::new);
        let new_head = Node { value, next };
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) {
        if let Some(c) = self.head.take() {
            self.head = c.next.map(|c| *c);
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0_usize;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_deref();
        }
        count
    }
}
