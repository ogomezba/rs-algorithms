#[derive(Debug)]
pub struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Self {
            adj: vec![Vec::new(); size],
        }
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].push(w);
        self.adj[w].push(v);
    }

    pub fn vertices(&self) -> usize {
        self.adj.len()
    }

    pub fn edges(&self) -> usize {
        self.adj.iter().fold(0, |acc, e| acc + e.len()) / 2
    }

    pub fn adj(&self, v: usize) -> &Vec<usize> {
        &self.adj[v]
    }

    pub fn len(&self) -> usize {
        self.adj.len()
    }
}
