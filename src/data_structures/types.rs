#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub next: Option<Box<Node<T>>>,
}
