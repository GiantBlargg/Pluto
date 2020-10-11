mod parser;

use parser::{Parser, Statement};
use std::{
	fs::File,
	io::{self, Write},
	path::PathBuf,
};

pub struct Assembler {
	data: Vec<u32>,
}

impl Assembler {
	pub fn new() -> Self {
		Self { data: Vec::new() }
	}
	pub fn load_file(self: &mut Self, in_path: &PathBuf) {
		let parser = Parser::new(File::open(in_path).unwrap());

		for s in parser {
			self.add_statement(s);
		}
	}
	fn add_statement(self: &mut Self, statement: Statement) {
		match statement {
			Statement::Function(func) => {
				self.data.push((func.args & 0xfff) << 12 + func.ret & 0xfff);
				for inst in func.block {
					self.data.push(inst.0 & 0xffffff);
				}
			}
		}
	}
	pub fn write(self: Self, mut out: File) -> io::Result<()> {
		let data = convert_24_bit(self.data);
		out.write_all(&data)?;
		Ok(())
	}
}

fn convert_24_bit(words: Vec<u32>) -> Vec<u8> {
	words
		.iter()
		.flat_map(|w| vec![(w >> 16) as u8, (w >> 8) as u8, (*w) as u8])
		.collect()
}
