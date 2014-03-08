use std::os;

fn main() {
	let args = os::args();


	for i in args.iter() {
		print!("{:s} ", *i);
	}

	println!("");
}
