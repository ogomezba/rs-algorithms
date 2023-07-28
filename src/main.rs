use std::time::Instant;

use crate::algorithms::{heap_sort::sort, knuth_shuffle::shuffle};

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let mut v: Vec<u32> = (0..=10000000).collect();
    let shuffling = Instant::now();
    shuffle(&mut v);
    println!("{:#?}", v.last());
    println!("{:#?}", shuffling.elapsed());

    let sorting = Instant::now();
    let sorted = sort(v);
    println!("{:#?}", sorted.last());
    println!("{:#?}", sorting.elapsed());
}
