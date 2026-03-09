use std::ffi::OsString;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
	/// A path to a csv file of edges.
	/// Each record `a,b,weight,name` describes an edge from node `a` to `b` with weight `weight` named `name`.
	pub csv_path: OsString,
}
