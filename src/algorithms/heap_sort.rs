use crate::data_structures::binary_heap::BinaryHeap;

pub fn sort<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    let mut bh = BinaryHeap::new();
    let mut sorted_v = Vec::with_capacity(v.len());

    for e in v.drain(0..) {
        bh.insert(e);
    }

    while !bh.is_empty() {
        sorted_v.push(bh.delete_max());
    }

    return sorted_v;
}
