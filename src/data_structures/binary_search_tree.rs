use std::cmp::Ordering;

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

    pub fn delete(&mut self, key: K) {
        self.root = Self::do_delete(self.root.take(), key);
    }

    fn do_delete(node: Option<Box<Node<K, T>>>, key: K) -> Option<Box<Node<K, T>>> {
        match node {
            None => None,
            Some(mut node) => {
                match key.cmp(&node.key) {
                    Ordering::Less => {
                        node.left = Self::do_delete(node.left, key);
                    }
                    Ordering::Greater => {
                        node.right = Self::do_delete(node.right, key);
                    }
                    Ordering::Equal => match (node.left.is_none(), node.right.is_none()) {
                        (true, true) => return None,
                        (true, false) => return node.right,
                        (false, true) => return node.left,
                        (false, false) => {
                            let (right, deleted) = Self::do_delete_min(node.right.unwrap());
                            let mut deleted = deleted.unwrap();
                            deleted.right = right;
                            deleted.left = node.left;
                            node = deleted;
                        }
                    },
                };

                node.size = 1 + Self::size(&node.left) + Self::size(&node.right);
                Some(node)
            }
        }
    }

    pub fn delete_min(&mut self) -> Option<K> {
        match self.root.take() {
            Some(node) => {
                let (root, deleted) = Self::do_delete_min(node);
                self.root = root;
                deleted.map(|n| n.key)
            }
            None => None,
        }
    }

    fn do_delete_min(
        mut node: Box<Node<K, T>>,
    ) -> (Option<Box<Node<K, T>>>, Option<Box<Node<K, T>>>) {
        if node.left.is_none() {
            (node.right.take(), Some(node))
        } else {
            let (left, deleted) = Self::do_delete_min(node.left.unwrap());
            node.left = left;
            node.size = 1 + Self::size(&node.left) + Self::size(&node.right);
            (Some(node), deleted)
        }
    }

    pub fn min(&self) -> Option<&K> {
        if self.root.is_none() {
            return None;
        }

        let mut node = self.root.as_deref().unwrap();

        while node.left.is_some() {
            node = node.left.as_deref().unwrap();
        }

        Some(&node.key)
    }

    pub fn max(&self) -> Option<&K> {
        if self.root.is_none() {
            return None;
        }

        let mut node = self.root.as_deref().unwrap();

        while node.right.is_some() {
            node = node.right.as_deref().unwrap();
        }

        Some(&node.key)
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |n| n.size)
    }

    pub fn rank(&self, key: K) -> usize {
        Self::do_rank(&self.root, key)
    }

    fn do_rank(node: &Option<Box<Node<K, T>>>, key: K) -> usize {
        match node.as_deref() {
            None => 0,
            Some(n) => match key.cmp(&n.key) {
                Ordering::Less => Self::do_rank(&n.left, key),
                Ordering::Greater => 1 + Self::size(&n.left) + Self::do_rank(&n.right, key),
                Ordering::Equal => Self::size(&n.left),
            },
        }
    }

    pub fn select(&self, k: usize) -> &K {
        Self::do_select(&self.root, k, 0)
    }

    fn do_select(node: &Option<Box<Node<K, T>>>, k: usize, acc: usize) -> &K {
        let node = node
            .as_deref()
            .expect("k can't be higher than the number of elements");

        let current_k = acc + 1 + Self::size(&node.left);

        match k.cmp(&current_k) {
            Ordering::Equal => &node.key,
            Ordering::Less => Self::do_select(&node.left, k, acc),
            Ordering::Greater => Self::do_select(&node.right, k, 1 + acc + Self::size(&node.left)),
        }
    }

    pub fn search_1d(&self, lo: &K, hi: &K) -> Vec<&K> {
        Self::do_search_1d(&self.root, lo, hi, Vec::new())
    }

    fn size(node: &Option<Box<Node<K, T>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn do_search_1d<'a>(
        node: &'a Option<Box<Node<K, T>>>,
        lo: &K,
        hi: &K,
        mut acc: Vec<&'a K>,
    ) -> Vec<&'a K> {
        match node.as_deref() {
            None => acc,
            Some(node) => {
                if &node.key >= lo && &node.key <= hi {
                    acc.push(&node.key);
                }

                if &node.key > lo {
                    acc = Self::do_search_1d(&node.left, lo, hi, acc);
                }

                if &node.key < hi {
                    acc = Self::do_search_1d(&node.right, lo, hi, acc);
                }

                acc
            }
        }
    }
}
