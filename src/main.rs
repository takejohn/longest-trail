use std::{error::Error, ffi::OsStr, fmt::Display, fs::File, io};

use crate::{args::ARGS, graph::WeightedDiGraph, solver::{find_longest_trails, save_trail_to_csv}, visualizer::run_visualizer};

mod visualizer;
mod args;
mod csv;
mod graph;
mod solver;

#[derive(Debug)]
struct UnknownNodeError(String);

impl Display for UnknownNodeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Unknown node: {}", &self.0)
	}
}

impl Error for UnknownNodeError {}

fn main() -> Result<(), Box<dyn Error>> {
	let graph = load_graph_from_csv_file(&ARGS.csv_path)?;
	run_visualizer(graph.graph())?;
	if let Some(start) = &ARGS.start {
		let Some(start_node) = graph.node(start) else {
			return Err(Box::new(UnknownNodeError(start.to_owned())));
		};
		let longest_trails = find_longest_trails(ARGS.algorithm, graph.graph(), start_node);
		if let Some(lonest_trail) = longest_trails.first() {
			save_trail_to_csv(lonest_trail.iter().map(|&ix| ix), graph.graph(), io::stdout())?;
		}
	}
	Ok(())
}

fn load_graph_from_csv_file(path: &OsStr) -> Result<WeightedDiGraph, Box<dyn Error>> {
	let file = File::open(path)?;
	let graph = WeightedDiGraph::load_from_csv(file)?;
	return Ok(graph)
}
