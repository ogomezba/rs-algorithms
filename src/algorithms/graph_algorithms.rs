use crate::data_structures::{graph::Graph, queue::Queue};

#[derive(Debug)]
pub struct DepthFirstPath {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    s: usize,
}

impl DepthFirstPath {
    pub fn create(g: &Graph, s: usize) -> Self {
        let mut marked = vec![false; g.len()];
        let mut edge_to = vec![0; g.len()];
        bfs(g, s, &mut marked, &mut edge_to);

        Self { marked, edge_to, s }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if !self.has_path_to(v) {
            return None;
        }

        let mut path = Vec::new();
        let mut w = v;

        while w != self.s {
            path.push(w);
            w = self.edge_to[w];
        }

        Some(path)
    }
}

fn bfs(g: &Graph, v: usize, marked: &mut Vec<bool>, edge_to: &mut Vec<usize>) {
    marked[v] = true;

    for &w in g.adj(v) {
        if !marked[w] {
            edge_to[w] = v;
            bfs(&g, w, marked, edge_to);
        }
    }
}

#[derive(Debug)]
pub struct BreadthFirstPath {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    s: usize,
}

impl BreadthFirstPath {
    pub fn create(g: &Graph, s: usize) -> Self {
        let mut marked = vec![false; g.len()];
        let mut edge_to = vec![0; g.len()];
        let mut queue = Queue::new();

        marked[s] = true;
        queue.enqueue(s);

        while let Some(v) = queue.dequeue() {
            for &w in g.adj(v) {
                if !marked[w] {
                    edge_to[w] = v;
                    marked[w] = true;
                    queue.enqueue(w);
                }
            }
        }

        Self { marked, edge_to, s }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if !self.has_path_to(v) {
            return None;
        }

        let mut path = Vec::new();
        let mut w = v;

        while w != self.s {
            path.push(w);
            w = self.edge_to[w];
        }

        path.reverse();
        Some(path)
    }
}
