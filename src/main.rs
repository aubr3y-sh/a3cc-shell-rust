use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");  // Display a prompt
        io::stdout().flush().expect("Failed to flush stdout");  // Make sure the prompt is shown

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();  // Remove any trailing newline

        if input == "exit" {
            break;  // Exit the shell if the user types "exit"
        }

        println!("You typed: {}", input);  // Print what the user typed
    }
}
