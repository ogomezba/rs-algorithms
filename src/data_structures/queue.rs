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
