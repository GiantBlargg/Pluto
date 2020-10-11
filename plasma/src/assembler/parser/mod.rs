mod grammar;

use grammar::statement;
use std::io::Read;

#[derive(Clone)]
pub struct OpCode(pub u32);

pub struct Function {
	pub args: u32,
	pub ret: u32,
	pub block: Vec<OpCode>,
}

pub enum Statement {
	Function(Function),
}

pub struct Parser<T: Read> {
	input: T,
	buffer: Vec<u8>,
}

impl<T: Read> Parser<T> {
	pub fn new(mut input: T) -> Self {
		let mut buffer = Vec::new();
		input.read_to_end(&mut buffer).unwrap();
		Self { input, buffer }
	}
}

impl<'a, T: Read> Iterator for Parser<T> {
	type Item = Statement;
	fn next(self: &mut Self) -> Option<Self::Item> {
		if self.buffer.len() == 0 {
			return None;
		}
		let (slice, st) = statement(self.buffer.as_slice()).unwrap();
		let mut new_buffer = Vec::new();
		new_buffer.resize(slice.len(), u8::default());
		new_buffer.copy_from_slice(slice);
		self.buffer = new_buffer;
		Some(st)
	}
}
