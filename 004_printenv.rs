use std::os;

fn main() {
	let e = os::env();

	for li in e.iter() {
		match *li {
			(ref cv, ref vv) => println!("{}={}", cv, vv)
		}
	}
}
