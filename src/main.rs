use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("dash-0.1$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}
