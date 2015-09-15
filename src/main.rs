// A terminal shell implementation in Rust 
use std::io;

fn main() {

		let mut cmd = String::new();
		println!("{}::","\u{019B}");
		io::stdin().read_line(&mut cmd)
		.ok()
		.expect("Error: Command Not Found");
		let cmd_slice = &cmd;
		
	
}