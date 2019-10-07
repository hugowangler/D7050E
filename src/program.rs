use std::{
	fs::File,
	path::Path,
	io::prelude::*,
	error::Error
};

use crate::{
	parse::program_parser::parse,
	interpreter::interp,
	value::Value,
};

pub fn run(path: &Path) -> Option<Value> {
	let display = path.display();
	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(e) => panic!("Could not open {}: {}", display, e.description())
	};

	let mut input = String::new();
	match file.read_to_string(&mut input) {
		Ok(_) => (),
		Err(e) => panic!("Could not read file: {:?}", e)
	}

	match parse(input) {
		Ok(parsed_prog) => {
			print!("parsed_prog = {:#?}", &parsed_prog);
			interp(parsed_prog)
		},
		Err(e) => panic!("Error while parsing program: {:?}", e)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn fn_sum() {
		let res = run(Path::new("tests/sum.txt"));
		assert_eq!(res, Some(Value::Number(15)))
	}
}