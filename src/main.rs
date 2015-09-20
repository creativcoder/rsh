// A terminal shell implementation in Rust 
use std::io;

/*enum GenericOperand<T> {
	Present(T),
	Empty,
}

use GenericOperand::Present;

struct Command<T> {
	name : &'static str,
	operand : Present(T),
}*/

fn main() {

	let mut cmd = String::new();
	/*let my_cmd = Command {name:"echo",operand:};*/
	loop {

	print!("{}::","\u{019B}");
	io::stdin().read_line(&mut cmd)
		.ok()
		.expect("Error: Command Not Found");
	let cmd_vec:Vec<&str> = cmd.split(" ").collect();

	/*for i in cmd_vec {
		println!("{:?}",i);
	}*/

	match cmd_vec[0] {
		"echo" => println!("{:?}",cmd_vec[1]),
		"quit" => break,
		_ => println!("Error: Command not found"),

	}

	}
}