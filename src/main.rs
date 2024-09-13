use nix::sys::wait;
use nix::unistd::{fork, ForkResult};
use std::io;
use std::io::Write;
use std::process;
use std::process::Command;

fn main() {
    loop {
        print!("dash-0.1$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("before match: {}", process::id());
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child: _ }) => {
                println!("parent: {}", process::id());
                wait::wait().expect("Couldn't wait for some reason");
                continue
            }
            Ok(ForkResult::Child) => {
                input.pop();
                let child = Command::new(input).spawn();
                let _output = child.expect("Error").wait_with_output();
                println!("child {}", process::id());
            }

            Err(_) => println!("Fork failed"),
        }
    }
}
