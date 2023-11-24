use std::{cmp::Reverse, collections::BinaryHeap};

use crate::data_structures::weighted_graph::{Edge, WeightedGraph};

use super::quick_union_weighted::QuickUnionWeighted;

pub fn kruskal_mst(g: WeightedGraph) -> (Vec<Edge>, u32) {
    let mut mst = Vec::with_capacity(g.vertices() - 1);
    let mut total_weight = 0;

    let mut qu = QuickUnionWeighted::new(mst.len());
    let mut pq = BinaryHeap::from_iter(g.edges().iter().map(Reverse));

    while let Some(Reverse(edge)) = pq.pop() {
        if mst.len() >= g.vertices() - 1 {
            break;
        }

        let (v, w) = edge.both();

        if !qu.connected(v, w) {
            qu.union(v, w);
            mst.push(Edge::clone(edge));
            total_weight += edge.weight();
        }
    }

    return (mst, total_weight);
}
