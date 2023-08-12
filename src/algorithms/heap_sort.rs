use crate::data_structures::binary_heap::min_priority_queue;

pub fn sort<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    let mut priority_queue = min_priority_queue();

    let mut sorted_v = Vec::with_capacity(v.len());

    for e in v.drain(0..) {
        priority_queue.insert(e);
    }

    while !priority_queue.is_empty() {
        sorted_v.push(priority_queue.delete_max());
    }

    return sorted_v;
}
