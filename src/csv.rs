use std::io;

use csv::{DeserializeRecordsIter, Reader};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Record {
	pub from: String,
	pub to: String,
	pub weight: u64,
	pub name: String,
}

pub struct CSVDeserializer<R: io::Read>(Reader<R>);

impl<R> CSVDeserializer<R> where R: io::Read {
	pub fn new(rdr: R) -> Self {
		CSVDeserializer(csv::ReaderBuilder::new()
			.has_headers(false)
			.from_reader(rdr))
	}

	pub fn deserialize(&mut self) -> DeserializeRecordsIter<'_, R, Record> {
		self.0.deserialize()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn deserialize_csv(csv: &str) -> Result<Vec<Record>, csv::Error> {
		CSVDeserializer::new(csv.as_bytes()).deserialize().collect()
	}

	fn record(from: impl Into<String>, to: impl Into<String>, weight: u64, name: impl Into<String>) -> Record {
		Record { from: from.into(), to: to.into(), weight, name: name.into() }
	}

	#[test]
	fn empty() {
		let csv = "";
		let records = deserialize_csv(csv).unwrap();
		assert_eq!(records, vec![]);
	}

	#[test]
	fn some_records() {
		let csv = "\
A,B,1,AB
A,C,2,AC
C,D,4,CD
D,B,5,DB
D,C,3,DB
";
		let records = deserialize_csv(csv).unwrap();
		assert_eq!(records, vec![
			record("A", "B", 1, "AB"),
			record("A", "C", 2, "AC"),
			record("C", "D", 4, "CD"),
			record("D", "B", 5, "DB"),
			record("D", "C", 3, "DB"),
		]);
	}
}
