mod assembler;

use assembler::Assembler;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
	source: PathBuf,
	#[structopt(short, long, parse(from_os_str))]
	output: Option<PathBuf>,
}

fn main() {
	let opt = Opt::from_args();

	let mut ass = Assembler::new();

	ass.load_file(&opt.source);

	ass.write(
		File::create(match opt.output {
			Some(o) => o,
			None => {
				let mut p = opt.source.clone();
				p.set_extension("plt");
				p
			}
		})
		.unwrap(),
	)
	.unwrap();
}
