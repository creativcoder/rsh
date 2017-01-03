
#[macro_use]
extern crate rustyline;
extern crate nix;
#[macro_use]
extern crate rsh;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use rsh::builtin::is_builtin;
use nix::unistd::{fork, execvp};
use nix::sys::wait::waitpid;
use nix::sys::wait::WaitStatus;
use nix::unistd::ForkResult;
use nix::Error;
use rsh::command::Command;
use nix::unistd::getcwd;

// On unix platforms you can use ANSI escape sequences
#[cfg(unix)]
static PROMPT: &'static str = "\x1b[1;32;mλ\x1b[0m ";

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("Command history unavailable");
    }
    loop {
        let cwd = getcwd().unwrap();
        let prompt = format!("[{}]::{}",cwd.to_str().unwrap(), PROMPT);
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                if line=="exit" {break;}
                rl.add_history_entry(&line);
                if !line.is_empty() {
                    let vec_cstring = Command::parse_cmd(&line);
                    if let Some(cmd) = is_builtin(&vec_cstring) {
                        cmd();
                        continue;
                    } else {
                	   let pid = fork();
                	   fork_and_exec(pid, vec_cstring);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("😏");
            },
            Err(ReadlineError::Eof) => {
                println!("Last job suspended.");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

fn fork_and_exec(pid: Result<ForkResult, Error>, command: Command) {
	match pid {
        Ok(ForkResult::Child) => {
            let Command { cmd, args} = command;
            execvp(&cmd, &args).unwrap();
        }
        Ok(ForkResult::Parent { child, .. }) => {
            // println!("in parent process with pid: {} and child pid:{}", getpid(), child);
            let wait_status = waitpid(child, None);
            match wait_status {
                // assert that waitpid returned correct status and the pid is the one of the child
                Ok(WaitStatus::Exited(pid, status)) =>  {
                    println!("child process with pid {} exited with status: {}", pid, status);
                },

                // panic, must never happen
                Ok(_) => panic!("Child still alive, should never happen"),

                // panic, waitpid should never fail
                Err(_) => panic!("Error: waitpid Failed")
            }

        },
        // panic, fork should never fail unless there is a serious problem with the OS
        Err(_) => panic!("Error: Fork Failed")
}
}