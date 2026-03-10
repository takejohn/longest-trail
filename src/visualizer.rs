use eframe::{NativeOptions, egui::{self, Pos2}, run_native};
use egui_graphs::{
	CenterGravity,
	Extra,
	FruchtermanReingoldWithCenterGravity,
	FruchtermanReingoldWithExtrasState,
	LayoutForceDirected,
	SettingsStyle,
	get_layout_state,
	set_layout_state,
	to_graph_custom,
};
use petgraph::{Directed, csr::DefaultIx};
use rand::RngExt;

use crate::{args::ARGS, graph::{Edge, Node, WeightedDiGraphInner}, visualizer::{edge::EdgeShape, font::create_font_definitions, node::NodeShape}};

mod edge;
mod font;
mod node;

type Graph = egui_graphs::Graph<Node, Edge, Directed, DefaultIx, NodeShape, EdgeShape>;
type L = LayoutForceDirected<FruchtermanReingoldWithCenterGravity>;
type S = FruchtermanReingoldWithExtrasState<(Extra<CenterGravity, true>, ())>;
type GraphView<'a> = egui_graphs::GraphView::<'a, Node, Edge, Directed, DefaultIx, NodeShape, EdgeShape, S, L>;

const SPAWN_SIZE: f32 = 250.;

pub struct GraphVisualizer {
	graph: Graph,
}

impl GraphVisualizer {
	pub fn new(_: &eframe::CreationContext<'_>, graph: &WeightedDiGraphInner) -> Self {
		let mut gui_graph = to_graph_custom(graph, node_transform, edge_transform);
		randomize_layout(&mut gui_graph);
		Self { graph: gui_graph }
	}
}

impl eframe::App for GraphVisualizer {
	fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let settings_style = SettingsStyle::new()
				.with_labels_always(true);
			let mut state = get_layout_state::<S>(ui, None);
			state.base.is_running = true;
			state.extras.0.params.c = ARGS.center_gravity;
			set_layout_state(ui, state, None);
			let mut graph_view = GraphView::new(&mut self.graph)
				.with_styles(&settings_style);
			ui.add(&mut graph_view);
		});
	}
}

pub fn run_visualizer(graph: &WeightedDiGraphInner) -> Result<(), eframe::Error> {
	if !ARGS.visualize {
		return Ok(());
	}

	run_native(
		"Graph",
		NativeOptions::default(),
		Box::new(|cc| {
			cc.egui_ctx.set_fonts(create_font_definitions());
			Ok(Box::new(GraphVisualizer::new(cc, graph)))
		})
	)
}

fn node_transform(node: &mut egui_graphs::Node<Node, Edge, Directed, DefaultIx, NodeShape>) {
	node.set_label(node.payload().to_string());
}

fn edge_transform(edge: &mut egui_graphs::Edge<Node, Edge, Directed, DefaultIx, NodeShape, EdgeShape>) {
	edge.set_label("".to_string());
}

fn randomize_layout(g: &mut Graph) {
	let mut rng = rand::rng();
	for node in g.g_mut().node_weights_mut() {
			node.set_location(Pos2::new(
					rng.random_range(0. ..SPAWN_SIZE),
					rng.random_range(0. ..SPAWN_SIZE),
			));
	}
}
