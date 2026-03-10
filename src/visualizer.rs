use eframe::{NativeOptions, egui, run_native};
use egui_graphs::{SettingsStyle, to_graph_custom};
use petgraph::Directed;

use crate::graph::{Edge, Node, WeightedDiGraphInner};

pub struct GraphVisualizer {
	graph: egui_graphs::Graph<Node, Edge, Directed>,
}

impl GraphVisualizer {
	pub fn new(_: &eframe::CreationContext<'_>, graph: &WeightedDiGraphInner) -> Self {
		Self { graph: to_graph_custom(graph, node_transform, edge_transform) }
	}
}

impl eframe::App for GraphVisualizer {
	fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let settings_style = SettingsStyle::new()
				.with_labels_always(true);
			let mut graph_view = egui_graphs::GraphView::<Node, Edge, Directed>::new(&mut self.graph)
				.with_styles(&settings_style);
			ui.add(&mut graph_view);
		});
	}
}

pub fn run_visualizer(graph: &WeightedDiGraphInner) -> Result<(), eframe::Error> {
	run_native(
		"Graph",
		NativeOptions::default(),
		Box::new(|cc| Ok(Box::new(GraphVisualizer::new(cc, graph))))
	)
}

pub fn node_transform(node: &mut egui_graphs::Node<Node, Edge>) {
	node.set_label(node.payload().to_string());
}

pub fn edge_transform(edge: &mut egui_graphs::Edge<Node, Edge>) {
	edge.set_label("".to_string());
}
