
use std::path::Path;
use std::ffi::CString;
use std::collections::HashMap;
use command::Command;

macro_rules! run_cmd {
	($cmd:expr) => ($cmd();)
}

fn say_hello() {
	println!("Hello world from rsh");
}

fn nop() {

}

pub fn gcd(a:u64,b:u64) -> u64 {
let mut a = a;
let mut b = b;
let mut r = a%b;
let mut temp = a;
loop {
		match r {
			0 => {return b;},
			_ => {temp=a;a=b;b=temp%b;r=a%b;},
		}
	}
}

pub fn is_builtin(cmd: &Command) -> Option<Box<Fn()>> {
	// referential destructuring
	let &Command {ref cmd, ref args} = cmd;
	
	match cmd.to_str().unwrap() {
		"hello" => return Some(Box::new(say_hello)),
		_ => None
	}
	
}

/*pub fn fib(rng: usize, init_val: u64) -> Option<u64> {
	let memo = Vec::<u64>::new();
	Some(0)
}

/// Finds the word count of the given path, 
pub fn wc(path: &Path, word: &str) -> Option<u64> {
	path.file_name().map_or(||{}, None)
}*/