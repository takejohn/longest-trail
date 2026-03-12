use std::{ffi::OsString, sync::LazyLock};

use clap::Parser;

use crate::solver::SolverAlgorithm;

#[derive(Debug, Parser)]
pub struct Args {
	/// A path to a csv file of edges.
	/// Each record `a,b,weight,name` describes an edge from node `a` to `b` with weight `weight` named `name`.
	pub csv_path: OsString,

	/// A node to start from.
	#[arg(short = 's', long)]
	pub start: Option<String>,

	/// an algorithm to solve the problem.
	#[arg(short = 'a', long, default_value_t)]
	pub algorithm: SolverAlgorithm,

	/// Open a window to show the graph.
	#[arg(short = 'v', long, default_value_t = false)]
	pub visualize: bool,

	/// (When visualize mode is on) Center gravity; pull nodes toward center.
	#[arg(long, default_value_t = 0.3)]
	pub center_gravity: f32,

	/// (When visualize mode is on) Size of a node and font.
	#[arg(long, default_value_t = 5.)]
	pub node_size: f32,

	/// (When visualize mode is on) Width of an edge.
	#[arg(long, default_value_t = 1.)]
	pub edge_width: f32,
}

pub static ARGS: LazyLock<Args> = LazyLock::new(|| {
	match Args::try_parse() {
			Ok(args) => args,
			Err(e) => e.exit(),
	}
});
