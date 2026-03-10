use std::{error::Error, ffi::OsStr, fs::File};

use crate::{args::ARGS, graph::WeightedDiGraph, visualizer::run_visualizer};

mod visualizer;
mod args;
mod csv;
mod graph;

fn main() -> Result<(), Box<dyn Error>> {
	let graph = load_graph_from_csv_file(&ARGS.csv_path)?;
	println!("nodes: {}", graph.node_count());
	println!("edges: {}", graph.edge_count());
	run_visualizer(graph.inner())?;
	Ok(())
}

fn load_graph_from_csv_file(path: &OsStr) -> Result<WeightedDiGraph, Box<dyn Error>> {
	let file = File::open(path)?;
	let graph = WeightedDiGraph::load_from_csv(file)?;
	return Ok(graph)
}
