use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        let output = Command::new(command)
            .args(&args)
            .output();

        match output {
            Ok(output) => {
                // Print the output from the command
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Err(e) => {
                println!("Error executing command: {}", e);
            }
        }
    }
}
