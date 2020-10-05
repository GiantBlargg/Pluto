use std::{fs::File, io::prelude::*, path::PathBuf};
use structopt::StructOpt;

mod vm;
use vm::PlutoVM;

#[derive(StructOpt)]
struct Opt {
	rom: PathBuf,
}

fn main() {
	let opt = Opt::from_args();

	let mut rom = Vec::new();
	File::open(opt.rom).unwrap().read_to_end(&mut rom).unwrap();

	let vm = PlutoVM::new(rom);
	vm.start();
}
