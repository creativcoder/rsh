

use rustyline::Editor;

pub struct Config {
	pub history: String,
	//pub bin_path: String add a binaries path
}

#[macro_export]
macro_rules! new_config {
	($it:ident, $file:expr) => {
		$it { history: $file.to_owned() }
	}
}
