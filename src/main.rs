use nix::sys::wait;
use nix::unistd::{fork, ForkResult};
use std::ffi::CString;
use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("dash-0.1$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = CString::new(input).expect("Failed to convert to CString");

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child: _ }) => {
                wait::wait().expect("Couldn't wait for some reason");
            }
            Ok(ForkResult::Child) => {
                let arg_test = CString::new("-a").expect("Failed to convert to CString");
                let path_test = CString::new("/usr/bin/").expect("Failed to convert to CString");
                let argument = vec![arg_test];
                let path = vec![path_test];

                nix::unistd::execve(&command, &path, &argument).expect("Command not found");
            }
            Err(_) => println!("Fork failed"),
        }
    }
}
