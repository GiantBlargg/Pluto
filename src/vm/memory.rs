struct MemoryBlock {
	contents: Vec<u32>,
	offset: u32,
	readable: bool,
	writeable: bool,
}
impl MemoryBlock {
	fn check(self: &Self, address: u32, write: bool) -> bool {
		address > self.offset
			&& address - self.offset < self.contents.len() as u32
			&& if write { self.writeable } else { self.readable }
	}
	fn read(self: &Self, address: u32) -> u32 {
		assert!(self.check(address, false));
		self.contents[(address - self.offset) as usize]
	}
	fn write(self: &mut Self, address: u32, value: u32) {
		assert!(self.check(address, true));
		self.contents[(address - self.offset) as usize] = value & 0xffffff;
	}
}
pub struct MemoryAccessor {
	mem_blocks: Vec<MemoryBlock>,
}
impl MemoryAccessor {
	pub fn new(mapping: u32, rom: Vec<u32>) -> Self {
		match mapping {
			0 => Self {
				mem_blocks: vec![MemoryBlock {
					contents: rom,
					offset: 0,
					readable: true,
					writeable: false,
				}],
			},
			_ => panic!("Unkown Mapping Mode"),
		}
	}
	fn get_memory_block(self: &mut Self, address: u32, write: bool) -> &mut MemoryBlock {
		self.mem_blocks
			.iter_mut()
			.filter(|b| b.check(address, write))
			.next()
			.unwrap()
	}
	pub fn read(self: &mut Self, address: u32) -> u32 {
		self.get_memory_block(address, false).read(address)
	}
	pub fn write(self: &mut Self, address: u32, value: u32) {
		self.get_memory_block(address, false).write(address, value)
	}
}
