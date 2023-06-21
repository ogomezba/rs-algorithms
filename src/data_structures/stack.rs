use super::types::Node;

#[derive(Debug)]
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { head: Option::None }
    }

    pub fn push(&mut self, elem: T) {
        let list = self.head.take();
        let new_node = Node { elem, next: list };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.elem)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
