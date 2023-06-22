use rand::Rng;

///This is not the ideal low-level implementation. In order to achive it, we would
///need to be able to create Arrays of a certain lenght. Because currently I'm
///not aware if that is possible in Rust (as the size of the element needs to
///be known at compile time), we are using directly a Vec and dealing with the
///random part of the algorithm. We are still going to treat Vec just like if it
///was a queue with the dynamic size implementation
///
///NOTE: Rust also has IteratorRandom

#[derive(Debug)]
pub struct RandomizedQueue<T> {
    elems: Vec<T>,
}

impl<T> RandomizedQueue<T> {
    pub fn new() -> Self {
        RandomizedQueue { elems: Vec::new() }
    }

    pub fn enqueue(&mut self, elem: T) {
        self.elems.push(elem);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.elems.len() == 0 {
            return None;
        }

        Some(self.elems.swap_remove(self.random_idx()))
    }

    pub fn sample(&self) -> Option<&T> {
        self.elems.get(self.random_idx())
    }

    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }

    pub fn size(&self) -> usize {
        self.elems.len()
    }

    pub fn iter(&self) -> RandomizedQueueIter<T> {
        RandomizedQueueIter {
            pending_indexes: (0..self.elems.len()).into_iter().collect(),
            queue_elems: &self.elems,
        }
    }

    fn random_idx(&self) -> usize {
        let mut rnd = rand::thread_rng();
        rnd.gen_range(0..self.elems.len())
    }
}

#[derive(Debug)]
pub struct RandomizedQueueIter<'a, T> {
    pending_indexes: Vec<usize>,
    queue_elems: &'a Vec<T>,
}

impl<'a, T> Iterator for RandomizedQueueIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pending_indexes.is_empty() {
            return None;
        }

        let mut rnd = rand::thread_rng();
        let idx = rnd.gen_range(0..self.pending_indexes.len());
        let n = self.pending_indexes.swap_remove(idx);

        Some(&self.queue_elems[n])
    }
}
