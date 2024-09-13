use nix::sys::wait;
use nix::unistd::{fork, ForkResult};
use std::io;
use std::io::Write;
use std::process::Command;
use sysinfo::Pid;

fn main() {
    loop {
        print!("dash-0.1$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let _pid = Pid::from_u32(0);

        match unsafe { fork() } {
            
            Ok(ForkResult::Parent { child: _pid }) => {

               wait::wait().expect("Couldn't wait for some reason");

            }
            Ok(ForkResult::Child) => {
                input.pop();
                let child = Command::new(input).spawn();
                let _output = child.expect("Error").wait_with_output();
            }

            Err(_) => println!("Fork failed"),
        }
    }
}
