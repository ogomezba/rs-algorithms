use rand::Rng;

use crate::algorithms::quick_union_weighted::QuickUnionWeighted;

pub fn execute_percolation(n: usize, runs: usize) {
    let mut rng = rand::thread_rng();
    let mut final_opens: Vec<usize> = Vec::with_capacity(runs);

    for _ in 0..runs {
        let mut percolator = Percolation::new(n);
        while !percolator.percolates() {
            let i = rng.gen_range(1..=n);
            let j = rng.gen_range(1..=n);

            percolator.open(i, j);
        }
        final_opens.push(percolator.number_of_open_sites());
    }

    let average_opens: f32 = (final_opens.iter().sum::<usize>() as f32) / (runs as f32);
    let total_elements: f32 = (n * n) as f32;

    println!(
        "The average proportion is: {}",
        average_opens / total_elements
    )
}

pub struct Percolation {
    qu: QuickUnionWeighted,
    open_sites: usize,
    elements: Vec<Vec<MatrixElement>>,
    virtual_origin: usize,
    virtual_end: usize,
}

struct MatrixElement {
    is_open: bool,
    idx: usize,
}

impl Percolation {
    pub fn new(n: usize) -> Self {
        let mut elements: Vec<Vec<MatrixElement>> = Vec::with_capacity(n);

        for i in 0..n {
            let mut row: Vec<MatrixElement> = Vec::with_capacity(n);
            for j in 0..n {
                let value = i * n + j;
                row.push(MatrixElement {
                    is_open: false,
                    idx: value,
                });
            }
            elements.push(row);
        }

        let qu = QuickUnionWeighted::new(n * n + 2);

        let virtual_origin = n * n;
        let virtual_end = n * n + 1;

        Percolation {
            qu,
            open_sites: 0,
            elements,
            virtual_origin,
            virtual_end,
        }
    }

    pub fn open(&mut self, i: usize, j: usize) {
        let MatrixElement { is_open, idx } = self.get_mut(i, j);

        if *is_open {
            return;
        }

        *is_open = true;
        let element = *idx;

        if i == 1 {
            self.qu.union(element, self.virtual_origin);
        }

        if i == self.elements.len() {
            self.qu.union(element, self.virtual_end);
        }

        if i > 1 {
            let top = self.get(i - 1, j);
            if top.is_open {
                self.qu.union(element, top.idx);
            }
        }

        if i < self.elements.len() {
            let bot = self.get(i + 1, j);
            if bot.is_open {
                self.qu.union(element, bot.idx);
            }
        }

        if j > 1 {
            let left = self.get(i, j - 1);
            if left.is_open {
                self.qu.union(element, left.idx);
            }
        }

        if j < self.elements.len() {
            let right = self.get(i, j + 1);
            if right.is_open {
                self.qu.union(element, right.idx);
            }
        }

        self.open_sites += 1;
    }

    fn number_of_open_sites(&self) -> usize {
        self.open_sites
    }

    fn percolates(&self) -> bool {
        self.qu.connected(self.virtual_origin, self.virtual_end)
    }

    fn get(&self, i: usize, j: usize) -> &MatrixElement {
        &self.elements[i - 1][j - 1]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut MatrixElement {
        &mut self.elements[i - 1][j - 1]
    }
}
