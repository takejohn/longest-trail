use std::io;

use csv::{DeserializeRecordsIter, Reader, Writer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
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

pub struct CSVSerialier<W: io::Write>(Writer<W>);

impl<W> CSVSerialier<W> where W: io::Write {
	pub fn new(wtr: W) -> Self {
		CSVSerialier(csv::WriterBuilder::new()
			.has_headers(false)
			.from_writer(wtr))
	}

	pub fn serialize(&mut self, records: impl IntoIterator<Item = Record>) -> csv::Result<()> {
		for record in records {
			self.0.serialize(record)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn record(from: impl Into<String>, to: impl Into<String>, weight: u64, name: impl Into<String>) -> Record {
		Record { from: from.into(), to: to.into(), weight, name: name.into() }
	}

	mod deserialize {
		use super::*;

		fn deserialize_csv(csv: &str) -> Result<Vec<Record>, csv::Error> {
			CSVDeserializer::new(csv.as_bytes()).deserialize().collect()
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

	mod serialize {
		use super::*;

		fn serialize_csv(records: Vec<Record>) -> Result<String, csv::Error> {
			let mut wtr = Vec::<u8>::new();
			CSVSerialier::new(&mut wtr).serialize(records)?;
			return Ok(String::from_utf8(wtr).unwrap());
		}

		#[test]
		fn empty() {
			let expected = "";
			let records = vec![];
			let csv = serialize_csv(records).unwrap();
			assert_eq!(csv, expected);
		}

		#[test]
		fn some_records() {
			let expected = "\
A,B,1,AB
A,C,2,AC
C,D,4,CD
D,B,5,DB
D,C,3,DB
";
			let records = vec![
				record("A", "B", 1, "AB"),
				record("A", "C", 2, "AC"),
				record("C", "D", 4, "CD"),
				record("D", "B", 5, "DB"),
				record("D", "C", 3, "DB"),
			];
			let csv = serialize_csv(records).unwrap();
			assert_eq!(csv, expected);
		}
	}
}
