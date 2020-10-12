mod parser;

use parser::{Address, Instruction, Parser, Statement};
use std::{
	collections::HashMap,
	fs::File,
	io::{self, Write},
	path::PathBuf,
};

struct Label {
	address: Option<u32>,
	repl_address: Vec<u32>,
}
impl Label {
	fn new() -> Self {
		Self {
			address: None,
			repl_address: Vec::new(),
		}
	}
}

pub struct Assembler {
	data: Vec<u32>,
	labels: HashMap<String, Label>,
}

impl Assembler {
	pub fn new() -> Self {
		Self {
			data: Vec::new(),
			labels: HashMap::new(),
		}
	}
	pub fn load_file(self: &mut Self, in_path: &PathBuf) {
		let parser = Parser::new(File::open(in_path).unwrap());

		for s in parser {
			self.add_statement(s);
		}

		for (_, record) in self.labels.iter() {
			for r in record.repl_address.iter() {
				self.data[*r as usize] = record.address.unwrap();
			}
		}
	}
	fn add_address(self: &mut Self, address: Address) {
		match address {
			Address::Const(n) => self.data.push(n),
			Address::Label(name) => {
				if !self.labels.contains_key(&name) {
					self.labels.insert(name.clone(), Label::new());
				}
				self.labels
					.get_mut(&name)
					.unwrap()
					.repl_address
					.push(self.data.len() as u32);
				self.data.push(0);
			}
		}
	}
	fn add_statement(self: &mut Self, statement: Statement) {
		match statement {
			Statement::Function(func) => {
				self.data
					.push(((func.args & 0xfff) << 12) + (func.ret & 0xfff));
				for inst in func.block {
					self.data.push(inst.get_opcode());
					if let Instruction::Push(a) = inst {
						self.add_address(a);
					}
				}
			}
			Statement::Skip(num) => {
				for _ in 0..num {
					self.data.push(0);
				}
			}
			Statement::SkipTo(address) => {
				let num = address as usize - self.data.len();
				if num > 0xffffff {
					panic!()
				}
				for _ in 0..num {
					self.data.push(0);
				}
			}
			Statement::Word(value) => self.add_address(value),
			Statement::Label(name) => {
				if !self.labels.contains_key(&name) {
					self.labels.insert(name.clone(), Label::new());
				}
				self.labels.get_mut(&name).unwrap().address = Some(self.data.len() as u32);
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
