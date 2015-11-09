mod builtin;
use std::io;
use builtin::*;

fn main() {

	let mut cmd = String::new();
	loop {

	print!("{}::","\u{019B}");
	io::stdin().read_line(&mut cmd)
		.ok()
		.expect("Error: Command Not Found");
		
	let cmd_vec:Vec<&str> = cmd.split(" ").collect();


	match cmd_vec[0] {
		"echo" => println!("{:?}",cmd_vec[1]),
		"gcd" => {//Todo},
		"quit" => break,
		_ => println!("Error: Command not found"),

	}

	}
}
