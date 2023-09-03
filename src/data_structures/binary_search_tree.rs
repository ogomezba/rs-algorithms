#[derive(Debug)]
struct Node<K: Ord, T> {
    key: K,
    val: T,
    left: Option<Box<Node<K, T>>>,
    right: Option<Box<Node<K, T>>>,
    size: usize,
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
        let mut current_node = self.root.as_deref();

        while let Some(node) = current_node {
            if node.key == key {
                return Some(&node.val);
            } else if key < node.key {
                current_node = node.left.as_deref();
            } else {
                current_node = node.right.as_deref();
            }
        }

        return None;
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
                    size: 1,
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

                node.size = 1
                    + node.left.as_ref().map_or(0, |n| n.size)
                    + node.right.as_ref().map_or(0, |n| n.size);
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

    pub fn max(&self) -> Option<&K> {
        let mut node = &self.root;
        let mut previous_node = None;

        while let Some(n) = node {
            previous_node = Some(n);
            node = &n.right;
        }

        previous_node.map(|n| &n.key)
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |n| n.size)
    }
}
