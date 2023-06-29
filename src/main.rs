use algorithms::knuth_shuffle::shuffle;

use crate::exercises::percolation::execute_percolation;

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let mut v: Vec<usize> = (1..21).collect();

    shuffle(&mut v);

    println!("{:#?}", v);

    execute_percolation(5, 1)
}
