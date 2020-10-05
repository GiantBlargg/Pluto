use std::iter::FromIterator;

mod func_execute;
use func_execute::FuncExecutor;

mod memory;
use memory::MemoryAccessor;

struct InteruptVectors {
	reset: u32,
}

struct PLTHeader {
	magic: u32,
	features: u32,
	mapping: u32,
	vectors: InteruptVectors,
	title: String,
	developer: String,
	publisher: String,
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
	String::from_iter(raw.iter().map(|c| std::char::from_u32(*c).unwrap_or('\0')))
}

fn convert_24_bit(bytes: Vec<u8>) -> Vec<u32> {
	bytes
		.chunks(3)
		.map(|i| u32::from_be_bytes([0, i[0], i[1], i[2]]))
		.collect()
}

pub struct PlutoVM {
	memory: MemoryAccessor,
	vectors: InteruptVectors,
	value_stack: Vec<u32>,
	function_stack: Vec<u32>,
}

impl PlutoVM {
	pub fn new(bytes: Vec<u8>) -> Self {
		let rom = convert_24_bit(bytes);
		let hdr = PLTHeader::create(&rom[0..0x40]);
		assert_eq!(hdr.magic, 0x504c54);
		assert_eq!(hdr.features, 0);
		print!(
			"Title:     {}\nDeveloper: {}\nPublisher: {}\n",
			hdr.title, hdr.developer, hdr.publisher
		);
		let memory = MemoryAccessor::new(hdr.mapping, rom);
		Self {
			memory,
			vectors: hdr.vectors,
			value_stack: Vec::new(),
			function_stack: Vec::new(),
		}
	}
	pub fn start(self: Self) {
		self._start();
	}
	fn _start(mut self: Self) {
		self.function_stack.push(self.vectors.reset);
		while let Some(func_ptr) = self.function_stack.pop() {
			let mut func_executor =
				FuncExecutor::new(&mut self.memory, func_ptr, &mut self.value_stack);

			loop {
				match func_executor.tick() {
					None => (),
					Some(mut f) => {
						self.function_stack.append(&mut f);
						break;
					}
				}
			}
		}

		if self.value_stack.len() > 0 {
			println!("Values left on the stack:");
			for i in self.value_stack.iter().rev() {
				println!("{}", i);
			}
		}
	}
}
