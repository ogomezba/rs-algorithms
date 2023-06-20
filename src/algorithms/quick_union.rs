pub struct QuickUnion {
    elements: Vec<usize>,
}

impl QuickUnion {
    pub fn new(number_of_elements: usize) -> Self {
        QuickUnion {
            elements: (0..number_of_elements).collect(),
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find_root(a);
        let root_b = self.find_root(b);

        let element_a = self.elements.get_mut(root_a).unwrap();
        *element_a = root_b;
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    fn find_root(&self, a: usize) -> usize {
        let v = self.elements.get(a).unwrap();

        if *v == a {
            a
        } else {
            self.find_root(*v)
        }
    }
}
