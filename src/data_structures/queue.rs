pub struct Queue<T> {
    elements: Vec<Option<T>>,
    first: usize,
    last: usize,
}

impl<T: Default> Queue<T> {
    pub fn new() -> Self {
        Queue {
            elements: Vec::new(),
            first: 0,
            last: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        if self.last == self.elements.len() || self.elements.is_empty() {
            self.elements.push(Some(item));
        } else {
            self.elements[self.last] = Some(item);
        }
        self.last += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.last == 0 {
            return None;
        }

        let item = std::mem::replace(&mut self.elements[self.first], None);
        self.first += 1;

        if self.first == self.last {
            self.first = 0;
            self.last = 0;
        }

        return item;
    }

    pub fn is_empty(&self) -> bool {
        self.last == 0
    }
}

// use std::mem::replace;
//
// use super::types::{Link, Node};
//
// #[derive(Debug)]
// pub struct Queue<'a, T> {
//     pub head: Link<T>,
//     last: &'a mut Link<T>,
// }
//
// impl<'a, T> Queue<'a, T> {
//     pub fn new() -> Self {
//         let mut head = Link::Empty;
//         Queue {
//             head,
//             last: &mut head,
//         }
//     }
//
//     pub fn enqueue(&mut self, elem: T) {
//         let new_node = Node {
//             elem,
//             next: Link::Empty,
//         };
//         let mut new_link = Link::More(Box::new(new_node));
//
//         match self.last {
//             Link::Empty => {
//                 self.head = new_link;
//             }
//             Link::More(node) => {
//                 node.next = new_link;
//                 self.last = &mut new_link;
//             }
//         }
//     }
//
//     pub fn dequeue(&mut self) -> Option<T> {
//         let link = replace(&mut self.head, Link::Empty);
//
//         match link {
//             Link::Empty => None,
//             Link::More(node) => {
//                 self.head = node.next;
//                 Some(node.elem)
//             }
//         }
//     }
// }
