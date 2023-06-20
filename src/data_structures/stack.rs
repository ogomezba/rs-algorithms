use std::mem::replace;

use super::types::{Link, Node};

#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let list = replace(&mut self.head, Link::Empty);
        let new_node = Node { elem, next: list };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let list = replace(&mut self.head, Link::Empty);
        match list {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            Link::Empty => true,
            Link::More(_) => false,
        }
    }
}
