#[derive(Debug)]
pub struct QuickUnionWeighted {
    pub elements: Vec<usize>,
    pub sizes: Vec<usize>,
}

impl QuickUnionWeighted {
    pub fn new(number_of_elements: usize) -> Self {
        let mut elements = Vec::with_capacity(number_of_elements);
        let mut sizes = Vec::with_capacity(number_of_elements);

        (0..number_of_elements).for_each(|e| {
            elements.push(e);
            sizes.push(1);
        });

        QuickUnionWeighted { elements, sizes }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        if a == b || self.connected(a, b) {
            return;
        }

        let root_a = self.find_root_mut(a);
        let root_b = self.find_root_mut(b);

        let size_a = self.sizes[root_a];
        let size_b = self.sizes[root_b];

        if size_a <= size_b {
            self.set_under(root_a, root_b);
            self.sizes[root_b] += size_a;
        } else {
            self.set_under(root_b, root_a);
            self.sizes[root_a] += size_b;
        }
    }

    pub fn set_under(&mut self, child: usize, parent: usize) {
        self.elements[child] = parent;
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    fn find_root(&self, a: usize) -> usize {
        let mut i = a;

        while i != self.elements[i] {
            i = self.elements[i];
        }

        return i;
    }

    fn find_root_mut(&mut self, a: usize) -> usize {
        let mut i = a;

        while i != self.elements[i] {
            self.elements[i] = self.elements[self.elements[i]];
            i = self.elements[i];
        }

        return i;
    }
}
