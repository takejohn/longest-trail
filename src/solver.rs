use std::{fmt::Display, io};

use clap::ValueEnum;
use petgraph::graph::{EdgeIndex, NodeIndex};

use crate::{csv::{CSVSerialier, Record}, graph::WeightedDiGraphInner};

pub use dfs::DFSSolver;

mod dfs;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SolverAlgorithm {
	DFS,
}

impl Default for SolverAlgorithm {
	fn default() -> Self {
		SolverAlgorithm::DFS
	}
}

impl Display for SolverAlgorithm {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			SolverAlgorithm::DFS => "dfs",
		})
	}
}

pub trait Solver {
	fn longest_trails(&self, graph: &WeightedDiGraphInner, start: NodeIndex) -> Vec<Vec<EdgeIndex>>;
}

pub fn find_longest_trails(algo: SolverAlgorithm, graph: &WeightedDiGraphInner, start: NodeIndex) -> Vec<Vec<EdgeIndex>> {
	match algo {
		SolverAlgorithm::DFS => DFSSolver.longest_trails(graph, start),
	}
}

pub fn save_trail_to_csv<I, W>(trail: I, graph: &WeightedDiGraphInner, wtr: W) -> csv::Result<()>
	where I: IntoIterator<Item = EdgeIndex>, W: io::Write
{
	CSVSerialier::new(wtr).serialize(trail.into_iter().map(|ix| {
		let (from, to) = graph.edge_endpoints(ix).expect("Edge should exist");
		let weight = graph.edge_weight(ix).expect("Edge should exist");
		Record {
			from: graph.node_weight(from).expect("Node should exist").name.as_ref().to_owned(),
			to: graph.node_weight(to).expect("Node should exist").name.as_ref().to_owned(),
			weight: weight.weight,
			name: weight.name.to_owned(),
		}
	}))
}
