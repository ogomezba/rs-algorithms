use std::usize;

use crate::data_structures::stack::Stack;

pub fn shell_sort<T: PartialOrd>(v: &mut Vec<T>) {
    let generator = ShellSequenceGenerator::new(v.len());

    for h in generator {
        shell_sort_h(v, h);
    }
}

pub fn shell_sort_h<T: PartialOrd>(v: &mut [T], h: usize) {
    if h < 1 || v.len() < h {
        return;
    }

    let mut i = h;

    while i < v.len() {
        let mut j = i;

        while j >= h {
            if v[j] < v[j - h] {
                v.swap(j, j - h);
                j -= h;
            } else {
                break;
            }
        }

        i += 1;
    }
}

struct ShellSequenceGenerator {
    stack: Stack<usize>,
}

impl ShellSequenceGenerator {
    fn new(max: usize) -> ShellSequenceGenerator {
        let mut x = 0;
        let mut stack = Stack::new();

        loop {
            let fx = 3 * x + 1;

            if fx > max {
                break;
            } else {
                stack.push(fx);
                x += 1;
            }
        }

        ShellSequenceGenerator { stack }
    }
}

impl Iterator for ShellSequenceGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
