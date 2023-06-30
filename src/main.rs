use std::time::Instant;

use algorithms::knuth_shuffle::shuffle;

use crate::{algorithms::merge_sort::sort, exercises::percolation::execute_percolation};

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let mut v = (0..=100000000).collect();
    let shuffling = Instant::now();
    shuffle(&mut v);
    println!("{:#?}", v.last());
    println!("{:#?}", shuffling.elapsed());

    let sorting = Instant::now();
    let sorted = sort(v);
    println!("{:#?}", sorted.last());
    println!("{:#?}", sorting.elapsed());

    execute_percolation(5, 1)
}
