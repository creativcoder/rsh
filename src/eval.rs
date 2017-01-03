use std::ffi::CString;

pub fn convert_args(line: &str) -> Vec<CString> {
	line.split(" ").map(|ch| CString::new(ch).unwrap()).collect::<Vec<CString>>()
}
