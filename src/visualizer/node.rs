use eframe::egui::{Pos2, Shape};
use egui_graphs::{DefaultNodeShape, DisplayNode, NodeProps};
use petgraph::{Directed, csr::DefaultIx};

use crate::graph::{Edge, Node};

#[derive(Debug, Clone)]
pub(super) struct NodeShape(DefaultNodeShape);

impl From<NodeProps<Node>> for NodeShape {
	fn from(value: NodeProps<Node>) -> Self {
		let mut inner = DefaultNodeShape::from(value);
		inner.radius = 16.;
		return NodeShape(inner);
	}
}

impl DisplayNode<Node, Edge, Directed, DefaultIx> for NodeShape {
	fn closest_boundary_point(&self, dir: eframe::egui::Vec2) -> eframe::egui::Pos2 {
		<DefaultNodeShape as DisplayNode<Node, Edge, Directed, DefaultIx>>::closest_boundary_point(&self.0, dir)
	}

	fn shapes(&mut self, ctx: &egui_graphs::DrawContext) -> Vec<eframe::egui::Shape> {
		let mut res = <DefaultNodeShape as DisplayNode<Node, Edge, Directed, DefaultIx>>::shapes(&mut self.0, ctx);
		let center = ctx.meta.canvas_to_screen_pos(self.0.pos);

		if let Some(shape) = res.get_mut(0) {
			*shape = Shape::Noop
		}

		if let Some(Shape::Text(shape)) = res.get_mut(1) {
			shape.pos = Pos2::new(center.x - shape.galley.size().x / 2., center.y - shape.galley.size().y / 2.);
		}

		return res;
	}

	fn update(&mut self, state: &egui_graphs::NodeProps<Node>) {
		<DefaultNodeShape as DisplayNode<Node, Edge, Directed, DefaultIx>>::update(&mut self.0, state)
	}

	fn is_inside(&self, pos: eframe::egui::Pos2) -> bool {
		<DefaultNodeShape as DisplayNode<Node, Edge, Directed, DefaultIx>>::is_inside(&self.0, pos)
	}
}
