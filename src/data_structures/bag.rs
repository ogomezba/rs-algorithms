pub struct Bag<T> {
    elements: Vec<T>,
}

impl<T> Bag<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, element: T) {
        self.elements.push(element)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.elements.iter()
    }
}
