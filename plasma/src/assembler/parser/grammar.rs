use super::{Address, Function, Instruction, Statement};
use nom::{
	alt,
	branch::alt,
	bytes::streaming::{tag, tag_no_case, take_while1},
	character::{
		is_alphabetic,
		streaming::{digit1, hex_digit1, multispace0},
	},
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
		value!($i, $opcode, tuple!(ws, tag_no_case!($name)))
	};
}

fn address_const(i: &[u8]) -> IResult<&[u8], Address> {
	let (r, n) = number(i)?;
	Ok((r, Address::Const(n)))
}
fn address_label(i: &[u8]) -> IResult<&[u8], Address> {
	let (r, (_, n)) = tuple((ws, label))(i)?;
	Ok((r, Address::Label(n)))
}
named!(address<Address>, alt!(address_const | address_label));

fn push(i: &[u8]) -> IResult<&[u8], Instruction> {
	let (r, (_, _, a)) = tuple((ws, tag_no_case("push"), address))(i)?;
	Ok((r, Instruction::Push(a)))
}

fn peek(i: &[u8]) -> IResult<&[u8], Instruction> {
	let (r, (_, _, n)) = tuple((ws, tag_no_case("peek"), number))(i)?;
	Ok((r, Instruction::Peek(n)))
}

named!(
	stack<Instruction>,
	alt!(
		push | simple_inst!("drop", Instruction::Drop)
			| simple_inst!("load", Instruction::Load)
			| peek | simple_inst!("stor", Instruction::Stor)
	)
);

named!(
	math<Instruction>,
	alt!(
		simple_inst!("neg", Instruction::Neg)
			| simple_inst!("add", Instruction::Add)
			| simple_inst!("sub", Instruction::Sub)
			| simple_inst!("mul", Instruction::Mul)
			| simple_inst!("udiv", Instruction::Udiv)
			| simple_inst!("sdiv", Instruction::Sdiv)
			| simple_inst!("mod", Instruction::Mod)
			| simple_inst!("rem", Instruction::Rem)
			| simple_inst!("not", Instruction::Not)
			| simple_inst!("and", Instruction::And)
			| simple_inst!("or", Instruction::Or)
			| simple_inst!("xor", Instruction::Xor)
	)
);

named!(
	comp<Instruction>,
	alt!(
		simple_inst!("eq", Instruction::Eq)
			| simple_inst!("ne", Instruction::Ne)
			| simple_inst!("ult", Instruction::Ult)
			| simple_inst!("slt", Instruction::Slt)
			| simple_inst!("ugt", Instruction::Ugt)
			| simple_inst!("sgt", Instruction::Sgt)
			| simple_inst!("ule", Instruction::Ule)
			| simple_inst!("sle", Instruction::Sle)
			| simple_inst!("uge", Instruction::Uge)
			| simple_inst!("sge", Instruction::Sge)
	)
);

named!(
	end<Instruction>,
	alt!(
		simple_inst!("ret", Instruction::Ret)
			| simple_inst!("jmp", Instruction::Jmp)
			| simple_inst!("if", Instruction::If)
			| simple_inst!("call", Instruction::Call)
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
	let (r, (_, v)) = tuple((tag_no_case("word"), address))(i)?;
	Ok((r, Statement::Word(v)))
}

fn label(i: &[u8]) -> IResult<&[u8], String> {
	let (r, s) = take_while1(|i| is_alphabetic(i) || i == b'_')(i)?;
	Ok((r, from_utf8(s).unwrap().to_string()))
}
fn label_def(i: &[u8]) -> IResult<&[u8], Statement> {
	let (r, (_, s)) = tuple((tag(":"), label))(i)?;
	Ok((r, Statement::Label(s)))
}

pub fn statement(i: &[u8]) -> IResult<&[u8], Statement> {
	let (r, (_, stat)) = tuple((ws, alt((function, skip, skip_to, word, label_def))))(i)?;
	Ok((r, stat))
}
