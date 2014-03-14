extern crate serialize;

// use std::io::stdout;
use std::io::stdin;
use serialize::json::Parser;

fn main() {
//	let mut o = stdout();
	let mut i = stdin();
	let mut jp = Parser::new(i.chars());
	
	match jp.parse() {
		Ok(j) => println!("{}", j),
		Err(e) => println!("{}", e)
	}	

}
