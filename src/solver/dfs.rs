use std::cmp::Ordering;

use petgraph::{graph::{EdgeIndex, NodeIndex}, visit::EdgeRef};

use crate::{graph::{Edge, WeightedDiGraphInner}, solver::Solver};

pub struct DFSSolver;

impl Solver for DFSSolver {
	fn longest_trails(&self, graph: &WeightedDiGraphInner, start: NodeIndex) -> Vec<Vec<EdgeIndex>> {
		let mut ctx = Context::new();
		dfs(graph, start, &mut ctx);
		return ctx.best_trails();
	}
}

#[derive(Debug, Clone)]
struct ComputedTrail {
	walk: Vec<(EdgeIndex, u64)>,
	sum: u64,
}

impl ComputedTrail {
	pub(self) fn new() -> Self {
		ComputedTrail { walk: Vec::new(), sum: 0, }
	}

	pub(self) fn push<E>(&mut self, edge_ref: &E) where E: EdgeRef<EdgeId = EdgeIndex, Weight = Edge> {
		let index = edge_ref.id();
		let weight = edge_ref.weight().weight;
		self.walk.push((index, weight));
		self.sum += weight;
	}

	pub(self) fn pop(&mut self) {
		if let Some((_, weight)) = self.walk.pop() {
			self.sum -= weight;
		}
	}

	pub(self) fn contains(&self, edge: EdgeIndex) -> bool {
		self.walk.iter().any(|(item, _)| *item == edge)
	}

	pub(self) fn sum(&self) -> u64 {
		self.sum
	}

	pub(self) fn indices(&self) -> Vec<EdgeIndex> {
		self.walk.iter().map(|(ix, _)| *ix).collect()
	}
}

struct Context {
	current_trail: ComputedTrail,
	best_trails: Vec<ComputedTrail>,
	best: u64,
}

impl Context {
	pub(self) fn new() -> Self {
		Self { current_trail: ComputedTrail::new(), best_trails: Vec::new(), best: 0 }
	}

	pub(self) fn push<E>(&mut self, edge_ref: &E) where E: EdgeRef<EdgeId = EdgeIndex, Weight = Edge> {
		self.current_trail.push(edge_ref);
		match self.current_trail.sum().cmp(&self.best) {
			Ordering::Less => {
				// nop
			}
			Ordering::Equal => {
				self.best_trails.push(self.current_trail.clone());
			}
			Ordering::Greater => {
				self.best = self.current_trail.sum;
				self.best_trails = vec![self.current_trail.clone()];
			}
		}
	}

	pub(self) fn pop(&mut self) {
		self.current_trail.pop();
	}

	pub(self) fn best_trails(&self) -> Vec<Vec<EdgeIndex>> {
		self.best_trails.iter().map(|trail| trail.indices()).collect()
	}
}

fn dfs(graph: &WeightedDiGraphInner, node: NodeIndex, ctx: &mut Context) -> u64 {
	for edge in graph.edges(node) {
		if !ctx.current_trail.contains(edge.id()) {
			ctx.push(&edge);
			dfs(graph, edge.target(), ctx);
			ctx.pop();
		}
	}
	return ctx.best;
}

#[cfg(test)]
mod tests {
	use crate::graph::WeightedDiGraph;

	use super::*;

	fn edge_indices_to_names<'a>(indices: &[EdgeIndex], graph: &'a WeightedDiGraphInner) -> Vec<&'a str> {
		let mut res = Vec::<&'a str>::with_capacity(indices.len());
		for &index in indices {
			res.push(&graph.edge_weight(index).unwrap().name);
		}
		return res;
	}

	#[test]
	fn one_edge() {
		let csv = "A,B,1,AB";
		let graph = WeightedDiGraph::load_from_csv(csv.as_bytes()).unwrap();
		let start = graph.node("A").unwrap();
		let longest_trails = DFSSolver.longest_trails(&graph.graph(), start);
		assert_eq!(longest_trails.len(), 1);
		assert_eq!(edge_indices_to_names(&longest_trails[0], graph.graph()), vec!["AB"]);
	}

	#[test]
	fn parallel_edges_equal() {
		let csv = "\
A,B,1,AB1
A,B,1,AB2
";
		let graph = WeightedDiGraph::load_from_csv(csv.as_bytes()).unwrap();
		let start = graph.node("A").unwrap();
		let longest_trails = DFSSolver.longest_trails(&graph.graph(), start);
		assert_eq!(longest_trails.len(), 2);
		assert_eq!(edge_indices_to_names(&longest_trails[0], graph.graph()), vec!["AB2"]);
		assert_eq!(edge_indices_to_names(&longest_trails[1], graph.graph()), vec!["AB1"]);
	}

	#[test]
	fn parallel_edges_not_equal() {
		let csv = "\
A,B,1,AB1
A,B,2,AB2
";
		let graph = WeightedDiGraph::load_from_csv(csv.as_bytes()).unwrap();
		let start = graph.node("A").unwrap();
		let longest_trails = DFSSolver.longest_trails(&graph.graph(), start);
		assert_eq!(longest_trails.len(), 1);
		assert_eq!(edge_indices_to_names(&longest_trails[0], graph.graph()), vec!["AB2"]);
	}

	#[test]
	fn equal_trails() {
		let csv = "\
A,B,1,AB
A,C,2,AC
B,D,4,BD
C,D,3,CD
";
		let graph = WeightedDiGraph::load_from_csv(csv.as_bytes()).unwrap();
		let start = graph.node("A").unwrap();
		let longest_trails = DFSSolver.longest_trails(&graph.graph(), start);
		assert_eq!(longest_trails.len(), 2);
		assert_eq!(edge_indices_to_names(&longest_trails[0], graph.graph()), vec!["AC", "CD"]);
		assert_eq!(edge_indices_to_names(&longest_trails[1], graph.graph()), vec!["AB", "BD"]);
	}

	#[test]
	fn self_cycle() {
		let csv = "A,A,1,AA";
		let graph = WeightedDiGraph::load_from_csv(csv.as_bytes()).unwrap();
		let start = graph.node("A").unwrap();
		let longest_trails = DFSSolver.longest_trails(&graph.graph(), start);
		assert_eq!(longest_trails.len(), 1);
		assert_eq!(edge_indices_to_names(&longest_trails[0], graph.graph()), vec!["AA"]);
	}

	#[test]
	fn cycle() {
		let csv = "\
A,B,1,AB
B,C,1,BC
C,A,1,CA
";
		let graph = WeightedDiGraph::load_from_csv(csv.as_bytes()).unwrap();
		let start = graph.node("A").unwrap();
		let longest_trails = DFSSolver.longest_trails(&graph.graph(), start);
		assert_eq!(longest_trails.len(), 1);
		assert_eq!(edge_indices_to_names(&longest_trails[0], graph.graph()), vec!["AB", "BC", "CA"]);
	}
}
