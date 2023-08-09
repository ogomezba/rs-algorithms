#[derive(Debug)]
struct Node<K: Ord, T> {
    key: K,
    val: T,
    left: Option<Box<Node<K, T>>>,
    right: Option<Box<Node<K, T>>>,
}

#[derive(Debug)]
pub struct BinarySearchTree<K: Ord, T> {
    root: Option<Box<Node<K, T>>>,
}

impl<K: Ord, T> BinarySearchTree<K, T> {
    pub fn new() -> BinarySearchTree<K, T> {
        BinarySearchTree { root: None }
    }

    pub fn get(&self, key: K) -> Option<&T> {
        let mut base_node = &self.root;
        loop {
            match base_node {
                None => return None,
                Some(node) => {
                    if node.key == key {
                        return Some(&node.val);
                    } else if key < node.key {
                        base_node = &node.left;
                    } else {
                        base_node = &node.right;
                    }
                }
            }
        }
    }

    pub fn put(&mut self, key: K, value: T) {
        self.root = Self::do_put(&mut self.root, key, value);
    }

    fn do_put(node: &mut Option<Box<Node<K, T>>>, key: K, val: T) -> Option<Box<Node<K, T>>> {
        match node {
            None => {
                return Some(Box::new(Node {
                    key,
                    val,
                    left: None,
                    right: None,
                }))
            }
            Some(node) => {
                if key < node.key {
                    node.left = Self::do_put(&mut node.left.take(), key, val);
                } else if key > node.key {
                    node.right = Self::do_put(&mut node.right.take(), key, val);
                } else {
                    node.val = val;
                }
            }
        }

        return node.take();
    }

    pub fn min(&self) -> Option<&K> {
        let mut node = &self.root;
        let mut previous_node = None;

        while let Some(n) = node {
            previous_node = Some(n);
            node = &n.left;
        }

        previous_node.map(|n| &n.key)
    }
}
