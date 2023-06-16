use std::env::args;

use percolation::execute_percolation;

mod percolation;
mod quick_find;
mod quick_union;
mod quick_union_weighted;

fn main() {
    let args: Vec<String> = args().collect();

    let n: usize = args[1].parse().unwrap();
    let runs: usize = args[2].parse().unwrap();
    execute_percolation(n, runs);
}
