use crate::{algorithms::shell_sort::shell_sort, exercises::percolation::execute_percolation};

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let mut v = vec![2, 4, 1, 8, 14, 1, 45, 789, 2];
    shell_sort(&mut v);
    println!("{:#?}", v);

    execute_percolation(5, 1)
}
