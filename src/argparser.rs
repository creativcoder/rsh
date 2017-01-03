pub fn parse(cmd: &str) -> Option<Vec<&str>> {
	Some(cmd.split(" ").collect::<Vec<&str>>())
}