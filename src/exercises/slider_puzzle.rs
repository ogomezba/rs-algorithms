use std::{fmt::Display, rc::Rc};

use crate::data_structures::binary_heap;

pub fn solve_puzzle(board: Board) -> SearchNode {
    let initial_search_node = SearchNode {
        board,
        moves: 0,
        parent: None,
    };

    let mut pq =
        binary_heap::BinaryHeap::new(|a: &SearchNode, b: &SearchNode| a.priority() > b.priority());

    pq.insert(initial_search_node);

    loop {
        let search_node = pq.delete_max();

        if search_node.board.is_goal() {
            return search_node;
        }

        let neighbors = search_node.board.neighbors();
        let search_node_ref = Rc::new(search_node);

        for neighbor in neighbors {
            let new_search_node = SearchNode {
                board: neighbor,
                moves: search_node_ref.moves + 1,
                parent: Some(Rc::clone(&search_node_ref)),
            };

            pq.insert(new_search_node);
        }
    }
}

pub struct SearchNode {
    board: Board,
    moves: usize,
    parent: Option<Rc<SearchNode>>,
}

impl SearchNode {
    fn priority(&self) -> usize {
        self.board.manhattan() + self.moves
    }

    pub fn print_solution(&self) {
        println!("{}", self.board);

        let mut next_node = self.parent.as_ref();

        while let Some(node) = next_node {
            println!("{}", node.board);
            next_node = node.parent.as_ref();
        }
    }
}

#[derive(Clone)]
pub struct Board {
    tiles: Vec<Vec<usize>>,
}

impl Board {
    pub fn new(tiles: Vec<Vec<usize>>) -> Self {
        Self { tiles }
    }

    pub fn dimension(&self) -> usize {
        self.tiles.len()
    }

    pub fn hamming(&self) -> usize {
        let mut hamming = 0;
        let mut i = 1;

        for row in &self.tiles {
            for n in row {
                if *n != i && *n != 0 {
                    hamming += 1;
                }

                i += 1;
            }
        }

        return hamming;
    }

    pub fn manhattan(&self) -> usize {
        let mut manhattan = 0;

        for i in 0..self.dimension() {
            for j in 0..self.dimension() {
                let tile = self.tiles[i][j];

                if tile != 0 {
                    let (goal_i, goal_j) = self.tile_goal(tile);
                    manhattan += goal_i.abs_diff(i) + goal_j.abs_diff(j);
                }
            }
        }

        return manhattan;
    }

    pub fn is_goal(&self) -> bool {
        self.hamming() == 0
    }

    pub fn neighbors(&self) -> Vec<Board> {
        let mut neighbors = Vec::with_capacity(4);
        let (zi, zj) = self.zero();

        if zi > 0 {
            let mut new_board = self.clone();
            new_board.swap((zi, zj), (zi - 1, zj));
            neighbors.push(new_board);
        }

        if zi < self.dimension() - 1 {
            let mut new_board = self.clone();
            new_board.swap((zi, zj), (zi + 1, zj));
            neighbors.push(new_board);
        }

        if zj > 0 {
            let mut new_board = self.clone();
            new_board.swap((zi, zj), (zi, zj - 1));
            neighbors.push(new_board);
        }

        if zj < self.dimension() - 1 {
            let mut new_board = self.clone();
            new_board.swap((zi, zj), (zi, zj + 1));
            neighbors.push(new_board);
        }

        return neighbors;
    }

    fn tile_goal(&self, n: usize) -> (usize, usize) {
        let n = n - 1; //Taking into account that the columns are a unit in advance
                       //with respect to the modulo

        (n / self.dimension(), n % self.dimension())
    }

    fn zero(&self) -> (usize, usize) {
        for row in 0..self.dimension() {
            for col in 0..self.dimension() {
                if self.tiles[row][col] == 0 {
                    return (row, col);
                }
            }
        }

        panic!("Invalid board. No 0 found");
    }

    fn swap(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        let aux = self.tiles[x1][y1];
        self.tiles[x1][y1] = self.tiles[x2][y2];
        self.tiles[x2][y2] = aux;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board: Vec<String> = self
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect();

        write!(f, "{}\n{}", self.tiles.len(), board.join("\n"))
    }
}
