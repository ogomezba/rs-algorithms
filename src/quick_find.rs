pub struct QuickFind {
    elements: Vec<usize>,
}

impl QuickFind {
    pub fn new(number_of_elements: usize) -> Self {
        QuickFind {
            elements: (0..number_of_elements).collect(),
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let v1 = *self.elements.get(a).unwrap();
        let v2 = *self.elements.get(b).unwrap();

        for v in self.elements.iter_mut() {
            if *v == v1 {
                *v = v2;
            }
        }
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        let v1 = self.elements.get(a).unwrap();
        let v2 = self.elements.get(b).unwrap();

        v1 == v2
    }
}
