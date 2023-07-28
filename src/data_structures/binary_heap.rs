#[derive(Debug)]
pub struct BinaryHeap<T: Ord> {
    elements: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn insert(&mut self, element: T) {
        self.elements.push(element);
        self.swim(self.elements.len() - 1);
    }

    pub fn delete_max(&mut self) -> T {
        let max = self.elements.swap_remove(0);
        self.sink(0);
        return max;
    }

    pub fn is_empty(&self) -> bool {
        return self.elements.is_empty();
    }

    fn swim(&mut self, i: usize) {
        let mut idx = i;

        while idx != 0 {
            let parent_idx = Self::parent(idx);

            let element = &self.elements[idx];
            let parent = &self.elements[parent_idx];

            if element <= parent {
                break;
            } else {
                self.elements.swap(idx, parent_idx);
                idx = parent_idx;
            }
        }
    }

    fn sink(&mut self, sink_start: usize) {
        let mut i = sink_start;

        loop {
            let mut child = self.first_child(i);

            if child >= self.elements.len() {
                return;
            }

            if child + 1 < self.elements.len() && self.elements[child + 1] > self.elements[child] {
                child += 1;
            }

            if self.elements[i] >= self.elements[child] {
                return;
            }

            self.elements.swap(i, child);
            i = child;
        }
    }

    fn parent(i: usize) -> usize {
        if i == 0 {
            return 0;
        }

        (i + 1) / 2 - 1
    }

    fn first_child(&self, i: usize) -> usize {
        return 2 * i + 1;
    }
}
