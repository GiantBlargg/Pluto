mod grammar;

use grammar::statement;
use std::io::Read;

#[derive(Clone)]
pub enum Address {
	Const(u32),
	Label(String),
}

#[derive(Clone)]
pub enum Instruction {
	Push(Address),
	Drop,
	Peek(u32),
	Load,
	Stor,
	Neg,
	Add,
	Sub,
	Mul,
	Udiv,
	Sdiv,
	Mod,
	Rem,
	Not,
	And,
	Or,
	Xor,
	Eq,
	Ne,
	Ult,
	Slt,
	Ugt,
	Sgt,
	Ule,
	Sle,
	Uge,
	Sge,
	Ret,
	Jmp,
	If,
	Call,
}

impl Instruction {
	pub fn get_opcode(self: &Self) -> u32 {
		match self {
			Instruction::Push(_) => 0x001000,
			Instruction::Drop => 0x001001,
			Instruction::Peek(x) => *x,
			Instruction::Load => 0x001002,
			Instruction::Stor => 0x001003,
			Instruction::Neg => 0x002000,
			Instruction::Add => 0x002001,
			Instruction::Sub => 0x002002,
			Instruction::Mul => 0x002003,
			Instruction::Udiv => 0x002005,
			Instruction::Sdiv => 0x002006,
			Instruction::Mod => 0x002008,
			Instruction::Rem => 0x002009,
			Instruction::Not => 0x00200b,
			Instruction::And => 0x00200c,
			Instruction::Or => 0x00200d,
			Instruction::Xor => 0x00200e,
			Instruction::Eq => 0x003000,
			Instruction::Ne => 0x003001,
			Instruction::Ult => 0x003002,
			Instruction::Slt => 0x003003,
			Instruction::Ugt => 0x003004,
			Instruction::Sgt => 0x003005,
			Instruction::Ule => 0x003006,
			Instruction::Sle => 0x003007,
			Instruction::Uge => 0x003008,
			Instruction::Sge => 0x003009,
			Instruction::Ret => 0x004000,
			Instruction::Jmp => 0x004001,
			Instruction::If => 0x004002,
			Instruction::Call => 0x004003,
		}
	}
}

pub struct Function {
	pub args: u32,
	pub ret: u32,
	pub block: Vec<Instruction>,
}

pub enum Statement {
	Function(Function),
	Skip(u32),
	SkipTo(u32),
	Word(Address),
	Label(String),
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
