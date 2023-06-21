use exercises::{dijkstra_arithmetics::evaluate, percolation::execute_percolation};

use crate::data_structures::stack_with_max::StackWithMax;

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let mut stack = StackWithMax::new();

    stack.push(3);
    stack.push(5);
    stack.push(2);
    stack.push(7);
    stack.push(10);
    stack.push(8);

    println!("{:#?}", stack);
    println!("{:#?}", stack.max());

    println!("{:#?}", stack.pop());

    println!("{:#?}", stack.max());
    stack.pop();
    stack.pop();

    println!("{:#?}", stack.max());

    println!("{}", evaluate("((3 + 5) * (1 + (3 * 7)))"));
    execute_percolation(5, 1)
}
