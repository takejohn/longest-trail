use std::{error::Error, ffi::OsStr, fs::File, io};

use clap::Parser;

use crate::{args::Args, graph::WeightedDiGraph};

mod args;
mod csv;
mod graph;

fn main() -> Result<(), Box<dyn Error>> {
	let args = match Args::try_parse() {
			Ok(args) => args,
			Err(e) => e.exit(),
	};
	let graph = load_graph_from_csv_file(&args.csv_path)?;
	println!("nodes: {}", graph.node_count());
	println!("edges: {}", graph.edge_count());
	Ok(())
}

fn load_graph_from_csv_file(path: &OsStr) -> Result<WeightedDiGraph, Box<dyn Error>> {
	let file = File::open(path)?;
	let graph = WeightedDiGraph::load_from_csv(file)?;
	return Ok(graph)
}
