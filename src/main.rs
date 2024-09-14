use nix::sys::wait;
use nix::unistd::{fork, ForkResult};
use std::io;
use std::io::Write;
use std::process;
use std::process::Command;
use sysinfo::Pid;

fn main() {
    loop {
        // printing the terminal cursor
        print!("dash-0.1$ ");
        io::stdout().flush().unwrap();

        // handling input and parsing
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();

        let command = input.split(" ");
        let cmd_vec: Vec<&str> = command.collect();

        // ignore execution on empty input
        if cmd_vec[0].trim().is_empty() {
            continue;
        }

        if cmd_vec[0] == "exit" {
            process::exit(0);
        }

        let _pid = Pid::from_u32(0);

        // fork processes to differenciate between native commands (todo) and external commands
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child: _pid }) => {

                wait::wait().expect("Couldn't wait for some reason");
            
            }
            Ok(ForkResult::Child) => {
                let child = Command::new(cmd_vec[0]).args(&cmd_vec[1..]).spawn();

                let _output = child
                    .expect("Error! Command not recognized.")
                    .wait_with_output();
                continue;
            }

            Err(_) => println!("Fork failed"),
        }
    }
}
