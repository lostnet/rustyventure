use std::os;

fn main() {
 let args = os::args();
 let argc = args.len();

 let foo = if argc > 3 { ~"ya"} else if argc > 1 {~"maybe"} else {~"nah"};
 println!("{:s}", foo);
}
