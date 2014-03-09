/*
 $ rustc -C prefer-dynamic 005_printenv_cgi.rs
 $ ldd ./005_printenv_cgi
    [build libdirectory from resolved paths]
 $ rustc -C prefer-dynamic  --sysroot . -L ./lib 005_printenv_cgi.rs
*/
use std::os;

#[start]
fn main(_:int, _:**u8) ->int {
	let e = os::env();

	println!("Content-Type: text/plain\n\n");

	for li in e.iter() {
		match *li {
			(ref cv, ref vv) => println!("{}={}", cv, vv)
		}
	}
	0
}
