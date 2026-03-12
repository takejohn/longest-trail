use std::{collections::HashMap, fmt::Display, io, rc::Rc};

use petgraph::{prelude::StableDiGraph, stable_graph::NodeIndex};

use crate::csv::CSVDeserializer;

#[derive(Debug, Clone)]
pub struct Node {
	pub name: Rc<str>,
}

impl Node {
	pub(self) fn new(name: Rc<str>) -> Self {
		Node { name: name.into() }
	}
}

impl Display for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

#[derive(Debug, Clone)]
pub struct Edge {
	pub weight: u64,
	pub name: String,
}

impl Edge {
	pub(self) fn new(weight: u64, name: String) -> Self {
		Edge { weight, name }
	}
}

impl Display for Edge {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\"{}\" {}", self.name, self.weight)
	}
}

pub type WeightedDiGraphInner = StableDiGraph<Node, Edge>;

type NameNodeIxMap = HashMap<Rc<str>, NodeIndex>;

pub struct WeightedDiGraph {
	inner: WeightedDiGraphInner,
	name_node_ix_map: NameNodeIxMap
}

impl WeightedDiGraph {
	pub fn load_from_csv<R>(rdr: R) -> Result<Self, csv::Error> where R: io::Read {
		let mut graph = WeightedDiGraphInner::new();
		let mut name_node_ix_map = NameNodeIxMap::new();

		for result in CSVDeserializer::new(rdr).deserialize() {
			let record = result?;
			let from = try_add_node(&mut graph, &mut name_node_ix_map, &record.from);
			let to = try_add_node(&mut graph, &mut name_node_ix_map, &record.to);
			let edge = Edge::new(record.weight, record.name);
			graph.add_edge(from, to, edge);
		}

		return Ok(WeightedDiGraph { inner: graph, name_node_ix_map });
	}

	pub fn graph(&self) -> &WeightedDiGraphInner {
		&self.inner
	}

	pub fn node(&self, name: &str) -> Option<NodeIndex> {
		self.name_node_ix_map.get(name).map(|&ix| ix)
	}
}

fn try_add_node(graph: &mut WeightedDiGraphInner, name_node_ix_map: &mut NameNodeIxMap, node_name: &str) -> NodeIndex {
	let existing = name_node_ix_map.get(node_name);
	match existing {
		Some(&ix) => ix,
		None => {
			let name = Rc::<str>::from(node_name);
			let node_ix = graph.add_node(Node::new(Rc::clone(&name)));
			name_node_ix_map.insert(name, node_ix);
			node_ix
		}
	}
}

#[cfg(test)]
mod tests {
  use super::*;

	#[test]
	fn empty() {
		let graph = WeightedDiGraph::load_from_csv("".as_bytes()).unwrap();
		assert_eq!(graph.graph().node_count(), 0);
		assert_eq!(graph.graph().edge_count(), 0);
	}

	#[test]
	fn some_records() {
		let csv = "\
A,B,1,AB
A,C,2,AC
C,D,4,CD
D,B,5,DB
D,C,3,DB
";
		let graph = WeightedDiGraph::load_from_csv(csv.as_bytes()).unwrap();
		assert_eq!(graph.graph().node_count(), 4);
		assert_eq!(graph.graph().edge_count(), 5);
	}

	mod node_reference {
		use super::*;

		#[test]
		fn weight() {
			let csv = "A,A,1,AA";
			let graph = WeightedDiGraph::load_from_csv(csv.as_bytes()).unwrap();
			let a = graph.node("A").and_then(|ix| graph.graph().node_weight(ix)).unwrap();
			assert_eq!(a.name.as_ref(), "A");
		}

		#[test]
		fn edges() {
			let csv = "A,B,1,AB";
			let graph = WeightedDiGraph::load_from_csv(csv.as_bytes()).unwrap();
			let a = graph.node("A").unwrap();
			let edges = graph.graph().edges(a).map(|r| r.weight()).collect::<Vec<&Edge>>();
			assert_eq!(edges.len(), 1);
			let edge = edges[0];
			assert_eq!(edge.name, "AB");
			assert_eq!(edge.weight, 1);
		}
	}
}
