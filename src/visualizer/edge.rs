use eframe::egui::{Pos2, Shape};
use egui_graphs::{DefaultEdgeShape, DisplayEdge, EdgeProps};
use petgraph::{Directed, csr::DefaultIx};

use crate::{args::ARGS, graph::{Edge, Node}, visualizer::NodeShape};

#[derive(Debug, Clone)]
pub(super) struct EdgeShape(DefaultEdgeShape);

impl From<EdgeProps<Edge>> for EdgeShape {
	fn from(value: EdgeProps<Edge>) -> Self {
		let mut inner = DefaultEdgeShape::from(value);
		inner.width = ARGS.edge_width;
		inner.tip_size = ARGS.edge_width * 5.;
		return EdgeShape(inner);
	}
}

impl DisplayEdge<Node, Edge, Directed, DefaultIx, NodeShape> for EdgeShape {
	fn shapes(
		&mut self,
		start: &egui_graphs::Node<Node, Edge, Directed, DefaultIx, NodeShape>,
		end: &egui_graphs::Node<Node, Edge, Directed, DefaultIx, NodeShape>,
		ctx: &egui_graphs::DrawContext,
	) -> Vec<Shape> {
		self.0.shapes(start, end, ctx)
	}

	fn update(&mut self, state: &EdgeProps<Edge>) {
		<DefaultEdgeShape as DisplayEdge<Node, Edge, Directed, DefaultIx, NodeShape>>::update(&mut self.0, state);
	}

	fn is_inside(
		&self,
		start: &egui_graphs::Node<Node, Edge, Directed, DefaultIx, NodeShape>,
		end: &egui_graphs::Node<Node, Edge, Directed, DefaultIx, NodeShape>,
		pos: Pos2,
	) -> bool {
		self.0.is_inside(start, end, pos)
	}
}
