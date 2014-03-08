use std::os;

fn main() {
 let args = os::args();
 let argc = args.len();

 let foo = if argc > 3 { "plenty"} else if argc > 1 {"some"} else {"nope"};

 println!("{:u} {:s}", argc, foo);
}
