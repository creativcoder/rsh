use std::ffi::CString;

use std::mem;

/// The result of parsing is a Command object
pub struct Command {
	pub cmd: CString,
	pub args: Vec<CString>
}

impl Command {
	pub fn parse_cmd(cmd_str: &str) -> Command {
		let cstring_vec = cmd_str.split(" ").map(|ch| CString::new(ch).unwrap()).collect::<Vec<CString>>();
		let (mut cmd, mut args) = cstring_vec.split_first().unwrap();
		println!("cmd:{:?} and args:{:?}",cmd, args );
		let mut args = args.to_vec();

		if args.is_empty() {
			// execvp fails if passed a NULL argv[0]
			args.push(CString::new(" ").unwrap());
		}
		Command {
			cmd: cmd.clone(),
			args: args
		}
	}
}