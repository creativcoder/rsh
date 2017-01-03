
use libc::pid_t as PID;

pub struct ProcessGroup {
	procs: Vec<PID>
}
