use std::rc::Rc;

pub struct WeightedGraph {
    edges: Vec<Vec<Rc<Edge>>>,
    edge_references: Vec<Rc<Edge>>,
}

impl WeightedGraph {
    pub fn new(n: usize) -> Self {
        let mut edges = Vec::new();

        for _ in 0..n {
            edges.push(Vec::new());
        }

        Self {
            edges,
            edge_references: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        let (v, w) = edge.both();
        let edge = Rc::new(edge);
        self.edge_references.push(edge.clone());
        self.edges[v].push(edge.clone());
        self.edges[w].push(edge);
    }

    pub fn adj(&self, v: usize) -> &[Rc<Edge>] {
        &self.edges[v]
    }

    pub fn vertices(&self) -> usize {
        self.edges.len() - 1
    }

    pub fn edges(&self) -> &[Rc<Edge>] {
        &self.edge_references
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Edge {
    v: usize,
    w: usize,
    weight: u32,
}

impl Edge {
    pub fn new(v: usize, w: usize, weight: u32) -> Self {
        Self { v, w, weight }
    }

    pub fn both(&self) -> (usize, usize) {
        (self.v, self.w)
    }

    pub fn other(&self, v: usize) -> usize {
        if self.v == v {
            self.w
        } else {
            self.v
        }
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}
