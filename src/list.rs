struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        return List { head: None };
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, value: T) {
        let new_head = Node {
            next: self.head.take(),
            value,
        };
        self.head = Some(Box::new(new_head));
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            let head = self.head.take().unwrap();
            self.head = head.next;
            return Some(head.value);
        }
        return None;
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.value)
    }
}
