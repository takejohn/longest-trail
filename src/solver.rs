use petgraph::graph::{EdgeIndex, NodeIndex};

use crate::graph::WeightedDiGraphInner;

mod pruning_dfs;

pub trait Solver {
	fn find_longest_trail(&self, graph: &WeightedDiGraphInner, start: NodeIndex) -> Vec<EdgeIndex>;
}
