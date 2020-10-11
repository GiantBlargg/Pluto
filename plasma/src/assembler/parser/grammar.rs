use super::{Function, OpCode, Statement};
use nom::{
	alt,
	branch::alt,
	bytes::streaming::tag_no_case,
	character::streaming::{digit1, hex_digit1, multispace0},
	is_not,
	multi::many0,
	named, pair,
	sequence::tuple,
	tag, tag_no_case, tuple, value, IResult,
};
use std::str::from_utf8;

named!(comment, value!(&[], pair!(tag!("#"), is_not!("\n\r"))));

named!(ws, alt!(multispace0 | comment));

fn decimal(i: &[u8]) -> IResult<&[u8], u32> {
	let (r, n) = digit1(i)?;
	Ok((r, u32::from_str_radix(from_utf8(n).unwrap(), 10).unwrap()))
}

fn hex(i: &[u8]) -> IResult<&[u8], u32> {
	let (r, (_, n)) = tuple((tag_no_case("0x"), hex_digit1))(i)?;
	Ok((r, u32::from_str_radix(from_utf8(n).unwrap(), 16).unwrap()))
}

fn number(i: &[u8]) -> IResult<&[u8], u32> {
	let (r, (_, n)) = tuple((ws, alt((hex, decimal))))(i)?;
	Ok((r, n))
}

macro_rules! simple_inst {
	($i:expr, $name:expr, $opcode:expr) => {
		value!($i, OpCode($opcode), tuple!(ws, tag_no_case!($name)))
	};
}

named!(
	stack<OpCode>,
	alt!(
		simple_inst!("drop", 0x001001)
			| simple_inst!("load", 0x001002)
			| simple_inst!("stor", 0x001003)
	)
);

named!(
	math<OpCode>,
	alt!(
		simple_inst!("neg", 0x002000)
			| simple_inst!("add", 0x002001)
			| simple_inst!("sub", 0x002002)
			| simple_inst!("mul", 0x002003)
			| simple_inst!("udiv", 0x002005)
			| simple_inst!("sdiv", 0x002006)
			| simple_inst!("mod", 0x002008)
			| simple_inst!("rem", 0x002009)
			| simple_inst!("not", 0x00200b)
			| simple_inst!("and", 0x00200c)
			| simple_inst!("or", 0x00200d)
			| simple_inst!("xor", 0x00200e)
	)
);

named!(
	comp<OpCode>,
	alt!(
		simple_inst!("eq", 0x003000)
			| simple_inst!("ne", 0x003001)
			| simple_inst!("ult", 0x003002)
			| simple_inst!("slt", 0x003003)
			| simple_inst!("ugt", 0x003004)
			| simple_inst!("sgt", 0x003005)
			| simple_inst!("ule", 0x003006)
			| simple_inst!("sle", 0x003007)
			| simple_inst!("uge", 0x003008)
			| simple_inst!("sge", 0x003009)
	)
);

named!(
	end<OpCode>,
	alt!(
		simple_inst!("ret", 0x004000)
			| simple_inst!("jmp", 0x004001)
			| simple_inst!("if", 0x004002)
			| simple_inst!("call", 0x004003)
	)
);

fn function(i: &[u8]) -> IResult<&[u8], Statement> {
	let (r, (_, args, ret, mut block, end)) = tuple((
		tag_no_case("func"),
		number,
		number,
		many0(alt((stack, math, comp))),
		end,
	))(i)?;
	block.push(end);
	Ok((r, Statement::Function(Function { args, ret, block })))
}

fn skip(i: &[u8]) -> IResult<&[u8], Statement> {
	let (r, (_, v)) = tuple((tag_no_case("skip"), number))(i)?;
	Ok((r, Statement::Skip(v)))
}
fn skip_to(i: &[u8]) -> IResult<&[u8], Statement> {
	let (r, (_, v)) = tuple((tag_no_case("skipto"), number))(i)?;
	Ok((r, Statement::SkipTo(v)))
}
fn word(i: &[u8]) -> IResult<&[u8], Statement> {
	let (r, (_, v)) = tuple((tag_no_case("word"), number))(i)?;
	Ok((r, Statement::Word(v)))
}

pub fn statement(i: &[u8]) -> IResult<&[u8], Statement> {
	let (r, (_, stat)) = tuple((ws, alt((function, skip, skip_to, word))))(i)?;
	Ok((r, stat))
}
