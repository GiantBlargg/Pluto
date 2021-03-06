mod func_execute;
mod memory;

use func_execute::FuncExecutor;
use memory::MemoryAccessor;
use std::iter::FromIterator;

pub struct InteruptVectors {
	reset: u32,
}

pub struct PLTHeader {
	pub magic: u32,
	pub features: u32,
	pub mapping: u32,
	pub vectors: InteruptVectors,
	pub title: String,
	pub developer: String,
	pub publisher: String,
}

impl PLTHeader {
	fn create(raw: &[u32]) -> Self {
		Self {
			magic: raw[0x0],
			features: raw[0x1],
			mapping: raw[0x2],
			vectors: InteruptVectors { reset: raw[0xf] },
			title: from_utf32(&raw[0x10..0x20]),
			developer: from_utf32(&raw[0x20..0x30]),
			publisher: from_utf32(&raw[0x30..0x40]),
		}
	}
}

fn from_utf32(raw: &[u32]) -> String {
	String::from_iter(
		raw.iter()
			.map(|c| std::char::from_u32(*c).unwrap_or('\0'))
			.take_while(|c| c != &'\0'),
	)
}

fn convert_24_bit(bytes: Vec<u8>) -> Vec<u32> {
	bytes
		.chunks(3)
		.map(|i| u32::from_be_bytes([0, i[0], i[1], i[2]]))
		.collect()
}

enum Func {
	Stack(Vec<u32>),
	Executor(FuncExecutor),
}

pub struct PlutoVM {
	memory: MemoryAccessor,
	pub header: PLTHeader,
	func: Func,
	function_stack: Vec<u32>,
}
impl PlutoVM {
	pub fn new(bytes: Vec<u8>) -> Self {
		let rom = convert_24_bit(bytes);
		let header = PLTHeader::create(&rom[0..0x40]);
		assert_eq!(header.magic, 0x504c54);
		assert_eq!(header.features, 0);
		print!(
			"Title:     {}\nDeveloper: {}\nPublisher: {}\n",
			header.title, header.developer, header.publisher
		);
		let memory = MemoryAccessor::new(header.mapping, rom);
		let function_stack = vec![header.vectors.reset];
		Self {
			memory,
			header,
			func: Func::Stack(Vec::new()),
			function_stack,
		}
	}
	pub fn tick(self: &mut Self) -> bool {
		let func_executor: &mut FuncExecutor = match &mut self.func {
			Func::Stack(stack) => {
				let func_ptr = match self.function_stack.pop() {
					Some(func_ptr) => func_ptr,
					None => {
						// Program Over
						if stack.len() > 0 {
							println!("Values left on the stack:");
							for i in stack.iter().rev() {
								println!("{}", i);
							}
						};
						return false;
					}
				};
				self.func = Func::Executor(FuncExecutor::new(
					self.memory.clone(),
					func_ptr,
					std::mem::take(stack),
				));
				match &mut self.func {
					Func::Executor(e) => e,
					Func::Stack(_) => panic!("The world doesn't make sense anymore."),
				}
			}
			Func::Executor(e) => e,
		};
		if !func_executor.tick() {
			let (mut func_stack, value_stack) =
				match std::mem::replace(&mut self.func, Func::Stack(Vec::new())) {
					Func::Executor(e) => e,
					Func::Stack(_) => panic!("The world doesn't make sense anymore."),
				}
				.dispose();
			self.function_stack.append(&mut func_stack);
			self.func = Func::Stack(value_stack);
		}
		true
	}
}
