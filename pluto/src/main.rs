mod vm;

use std::{fs::File, io::prelude::*, path::PathBuf};
use structopt::StructOpt;
use vm::PlutoVM;

#[derive(StructOpt)]
struct Opt {
	#[structopt(parse(from_os_str))]
	rom: PathBuf,
}

fn main() {
	let opt = Opt::from_args();

	let mut rom = Vec::new();
	File::open(opt.rom).unwrap().read_to_end(&mut rom).unwrap();

	Runtime::new(rom).run();
}

struct Runtime {
	vm: PlutoVM,
}
impl Runtime {
	fn new(rom: Vec<u8>) -> Self {
		let vm = PlutoVM::new(rom);
		Self { vm }
	}
	fn run(mut self: Self) {
		while self.vm.tick() {}
	}
}
