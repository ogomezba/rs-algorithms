use std::{fmt::Display, mem::uninitialized};

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

        for i in 0..self.tiles.len() {
            for j in 0..self.tiles.len() {
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

    pub fn neighbors(&self) -> NeighborsIterator {
        todo!()
    }

    fn tile_goal(&self, n: usize) -> (usize, usize) {
        let n = n - 1; //Taking into account that the columns are a unit in advance
                       //with respect to the modulo

        (n / self.dimension(), n % self.dimension())
    }
}

struct NeighborsIterator {}

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
