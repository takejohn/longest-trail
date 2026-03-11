use fixedbitset::FixedBitSet;
use petgraph::{graph::{EdgeIndex, NodeIndex}, visit::{EdgeIndexable, EdgeRef}};

use crate::{graph::{Edge, WeightedDiGraphInner}, solver::Solver};

pub(super) struct PruningDFSSolver;

impl Solver for PruningDFSSolver {
	fn find_longest_trail(&self, graph: &WeightedDiGraphInner, start: NodeIndex) -> Vec<EdgeIndex> {
		todo!()
	}
}

struct ComputedTrail {
	walk: Vec<EdgeIndex>,
	length: u64,
}

impl ComputedTrail {
	pub(self) fn new() -> Self {
		ComputedTrail { walk: vec![], length: 0, }
	}

	pub(self) fn append<E>(edge_ref: &E) where E: EdgeRef<_, _, Edge> {
	}
}

struct Context {
	used_edges: FixedBitSet,
	best: u64,
}

impl Context {
	pub(self) fn new(graph: &WeightedDiGraphInner) -> Self {
		Context { used_edges: FixedBitSet::with_capacity(graph.capacity().1), best: 0u64 }
	}
}

fn dfs(graph: &WeightedDiGraphInner, node: NodeIndex, ctx: &mut Context) -> u64 {
	for edge in graph.edges(node) {
		if !ctx.used_edges.contains(edge.id().index()) {
			ctx.used_edges.set(edge.id().index(), true);
			ctx.best = u64::max(ctx.best, edge.weight().weight + dfs(graph, node, ctx));
			ctx.used_edges.set(edge.id().index(), false);
		}
	}
	return ctx.best;
}
