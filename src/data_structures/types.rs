#[derive(Debug)]
pub enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}
