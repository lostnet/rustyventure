use std::io::stdout;
use std::io::stdin;

fn main() {
	let mut o = stdout();
	let mut i = stdin();

	
	while match i.read_u8() {
		Ok(c) => o.write_u8(c).is_ok(),
		Err(_)  => false
	} {}
	

}
