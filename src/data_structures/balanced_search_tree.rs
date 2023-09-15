use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug)]
pub struct BalancedSearchTree<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

#[derive(Debug)]
struct Node<K, V> {
    pub key: K,
    pub val: V,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
    pub color: Color,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Black,
}

impl<K: Ord + Debug, V: Debug> BalancedSearchTree<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut current_node = self.root.as_deref();

        while let Some(node) = current_node {
            match key.cmp(&node.key) {
                Ordering::Equal => return Some(&node.val),
                Ordering::Less => current_node = node.left.as_deref(),
                Ordering::Greater => current_node = node.right.as_deref(),
            }
        }

        return None;
    }

    pub fn put(&mut self, key: K, val: V) {
        self.root = Self::do_put(self.root.take(), key, val);
    }

    fn do_put(node: Option<Box<Node<K, V>>>, key: K, val: V) -> Option<Box<Node<K, V>>> {
        let mut node = match node {
            None => Box::new(Node {
                key,
                val,
                left: None,
                right: None,
                color: Color::Red,
            }),
            Some(mut node) => {
                if key == node.key {
                    node.val = val;
                    node
                } else if key < node.key {
                    node.left = Self::do_put(node.left, key, val);
                    node
                } else {
                    node.right = Self::do_put(node.right, key, val);
                    node
                }
            }
        };

        if Self::is_red(&node.right.as_ref()) && !Self::is_red(&node.left.as_ref()) {
            node = Self::rotate_left(*node);
        }

        if Self::is_red(&node.left.as_ref())
            && Self::is_red(&node.left.as_ref().and_then(|n| n.left.as_ref()))
        {
            node = Self::rotate_right(*node);
        }

        if Self::is_red(&node.left.as_ref()) && Self::is_red(&node.right.as_ref()) {
            node = Self::flip_colors(*node);
        }

        Some(node)
    }

    fn rotate_left(mut node: Node<K, V>) -> Box<Node<K, V>> {
        let mut x = node
            .right
            .expect("Failed when rotating left a node that had no right element");
        x.color = node.color;
        node.right = x.left;
        node.color = Color::Red;
        x.left = Some(Box::new(node));

        x
    }

    fn rotate_right(mut node: Node<K, V>) -> Box<Node<K, V>> {
        let mut x = node
            .left
            .expect("Failed when rotating right a node that had no left element");
        x.color = node.color;
        node.left = x.right;
        node.color = Color::Red;
        x.right = Some(Box::new(node));

        x
    }

    fn flip_colors(mut node: Node<K, V>) -> Box<Node<K, V>> {
        node.color = Color::Red;
        node.left = node.left.map(|mut node| {
            node.color = Color::Black;
            node
        });
        node.right = node.right.map(|mut node| {
            node.color = Color::Black;
            node
        });

        Box::new(node)
    }

    fn is_red(node: &Option<&Box<Node<K, V>>>) -> bool {
        node.as_ref().map_or(false, |node| node.color == Color::Red)
    }
}
