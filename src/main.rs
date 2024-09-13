// use nix::sys::wait;
// use nix::unistd::{fork, ForkResult};
use std::io;
use std::io::Write;
// use std::process;
use std::process::Command;

fn main() {
    loop{
        print!("dash-0.1$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        input.pop();

        let command = &input;

        {
        Command::new(command)
            .spawn()
            .expect("Command not found.");
        io::stdout().flush().unwrap();
        }
    }
}
