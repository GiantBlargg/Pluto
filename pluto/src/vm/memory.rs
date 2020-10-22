use std::{cell::RefCell, rc::Rc};

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

#[derive(Clone)]
pub struct MemoryAccessor {
	mem_blocks: Rc<RefCell<Vec<MemoryBlock>>>,
}
impl MemoryAccessor {
	pub fn new(mapping: u32, rom: Vec<u32>) -> Self {
		let mem_blocks = match mapping {
			0 => vec![MemoryBlock {
				contents: rom,
				offset: 0,
				readable: true,
				writeable: false,
			}],
			_ => panic!("Unkown Mapping Mode"),
		};
		Self {
			mem_blocks: Rc::new(RefCell::new(mem_blocks)),
		}
	}
	pub fn read(self: &Self, address: u32) -> u32 {
		self.mem_blocks
			.borrow()
			.iter()
			.filter(|b| b.check(address, false))
			.next()
			.unwrap()
			.read(address)
	}
	pub fn write(self: &mut Self, address: u32, value: u32) {
		self.mem_blocks
			.borrow_mut()
			.iter_mut()
			.filter(|b| b.check(address, true))
			.next()
			.unwrap()
			.write(address, value)
	}
}
