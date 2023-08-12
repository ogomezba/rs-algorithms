pub struct BinaryHeap<T> {
    pub elements: Vec<T>,
    less_eq: fn(&T, &T) -> bool,
}

impl<T> BinaryHeap<T> {
    pub fn new(less_eq: fn(&T, &T) -> bool) -> Self {
        Self {
            elements: Vec::new(),
            less_eq,
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

            if (self.less_eq)(element, parent) {
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

            if child + 1 < self.elements.len()
                && (self.less_eq)(&self.elements[child], &self.elements[child + 1])
            {
                child += 1;
            }

            if (self.less_eq)(&self.elements[child], &self.elements[i]) {
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

pub fn max_priority_queue<T: Ord>() -> BinaryHeap<T> {
    BinaryHeap::new(|a: &T, b: &T| a <= b)
}

pub fn min_priority_queue<T: Ord>() -> BinaryHeap<T> {
    BinaryHeap::new(|a: &T, b: &T| a > b)
}
