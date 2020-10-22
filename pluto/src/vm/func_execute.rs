use super::memory::MemoryAccessor;

struct StackAccess {
	stack: Vec<u32>,
	disallowed: usize,
	retc: usize,
}
impl StackAccess {
	fn decompose_sig(func_sig: u32) -> (u32, u32) {
		(func_sig >> 12, func_sig & 0xfff)
	}
	fn new(stack: Vec<u32>, func_sig: u32) -> StackAccess {
		let (argc, retc) = Self::decompose_sig(func_sig);
		let disallowed = stack.len() - argc as usize;
		assert!(
			(argc as usize) <= stack.len(),
			"Not enough args on the stack"
		);
		StackAccess {
			stack,
			disallowed,
			retc: retc as usize,
		}
	}

	fn pop(self: &mut Self) -> u32 {
		assert!(
			self.stack.len() > self.disallowed,
			"Function popped too much"
		);
		self.stack.pop().unwrap()
	}
	fn push(self: &mut Self, value: u32) {
		self.stack.push(value)
	}
	fn push_bool(self: &mut Self, value: bool) {
		self.push(if value { 1 } else { 0 })
	}
	fn peek(self: &Self, depth: u32) -> u32 {
		assert!(self.stack_height() > depth as usize);
		self.stack[self.stack.len() - 1 - depth as usize]
	}
	fn stack_height(self: &Self) -> usize {
		self.stack.len() - self.disallowed
	}
	fn compat_with(self: &Self, funcs: Vec<u32>) -> bool {
		self.stack_height() as isize
			+ funcs
				.iter()
				.map(|func| {
					let (argc, retc) = Self::decompose_sig(*func);
					retc as isize - argc as isize
				})
				.fold(0, |a, b| a + b)
			== self.retc as isize
	}
	fn dispose(self: Self) -> Vec<u32> {
		self.stack
	}
}

pub struct FuncExecutor {
	memory: MemoryAccessor,
	prg_ptr: u32,
	stack_access: StackAccess,
	func_stack: Option<Vec<u32>>,
}
impl FuncExecutor {
	pub fn new<'b>(memory: MemoryAccessor, func_ptr: u32, value_stack: Vec<u32>) -> FuncExecutor {
		let prg_ptr = func_ptr + 1;

		let func_sig = memory.read(func_ptr);
		let stack_access = StackAccess::new(value_stack, func_sig);

		FuncExecutor {
			memory,
			prg_ptr,
			stack_access,
			func_stack: None,
		}
	}
	pub fn tick(self: &mut Self) -> bool {
		let inst = self.memory.read(self.prg_ptr);

		if inst & 0xfff000 == 0 {
			self.stack_access.push(self.stack_access.peek(inst & 0xfff));
		} else {
			match inst {
				// Stack Manipulation
				0x001000 => {
					// push
					self.prg_ptr = self.prg_ptr + 1;
					self.stack_access.push(self.memory.read(self.prg_ptr));
				}
				0x001001 => {
					// drop
					self.stack_access.pop();
				}
				0x001002 => {
					// load
					let a = self.stack_access.pop();
					self.stack_access.push(self.memory.read(a));
				}
				0x001003 => {
					// stor
					let a = self.stack_access.pop();
					let v = self.stack_access.pop();
					self.memory.write(a, v);
				}

				// Math
				0x002000 => {
					// neg
					let x = self.stack_access.pop();
					self.stack_access.push_bool(x == 0)
				}
				0x002001 => {
					// add
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(x + y)
				}
				0x002002 => {
					// sub
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(x - y);
				}
				0x002003 => {
					// mul
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(x * y);
				}
				0x002005 => {
					// udiv
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(x / y);
				}
				0x002006 => {
					// udiv
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(((x as i32) / (y as i32)) as u32);
				}
				0x002008 => {
					// mod
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(x % y);
				}
				0x002009 => {
					// rem
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(((x as i32) % (y as i32)) as u32);
				}
				0x00200b => {
					// not
					let x = self.stack_access.pop();
					self.stack_access.push(!x);
				}
				0x00200c => {
					// and
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(x & y);
				}
				0x00200d => {
					// or
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(x | y);
				}
				0x00200e => {
					// xor
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push(x ^ y);
				}

				// Comparisons
				0x003000 => {
					// eq
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool(x == y);
				}
				0x003001 => {
					//ne
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool(x != y);
				}
				0x003002 => {
					// ult
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool(x < y);
				}
				0x003003 => {
					// slt
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool((x as i32) < (y as i32));
				}
				0x003004 => {
					// ugt
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool(x > y);
				}
				0x003005 => {
					// sgt
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool((x as i32) > (y as i32));
				}
				0x003006 => {
					// ule
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool(x <= y);
				}
				0x003007 => {
					// sle
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool((x as i32) <= (y as i32));
				}
				0x003008 => {
					// uge
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool(x >= y);
				}
				0x003009 => {
					// sge
					let y = self.stack_access.pop();
					let x = self.stack_access.pop();
					self.stack_access.push_bool((x as i32) >= (y as i32));
				}

				// End of functio
				0x004000 => {
					// ret
					self.func_stack = Some(Vec::new());
				}
				0x004001 => {
					// jmp
					self.func_stack = Some(vec![self.stack_access.pop()])
				}
				0x004002 => {
					// if
					let f1 = self.stack_access.pop();
					let f2 = self.stack_access.pop();
					let t = self.stack_access.pop();
					self.func_stack = Some(vec![if t == 0 { f2 } else { f1 }])
				}
				0x004003 => {
					// call
					let f1 = self.stack_access.pop();
					let f2 = self.stack_access.pop();
					self.func_stack = Some(vec![f2, f1]);
				}

				_ => panic!("Unkown opcode {}, at {}", inst, self.prg_ptr),
			}
		}
		match &self.func_stack {
			None => {
				self.prg_ptr = self.prg_ptr + 1;
				true
			}
			Some(f) => {
				let sigs = f
					.iter()
					.map(|func_ptr| self.memory.read(*func_ptr))
					.collect();
				assert!(
					self.stack_access.compat_with(sigs),
					"Wrong number of returns"
				);
				false
			}
		}
	}
	pub fn dispose(self: Self) -> (Vec<u32>, Vec<u32>) {
		(self.func_stack.unwrap(), self.stack_access.dispose())
	}
}
