use std::{io::{self, Write}, process};

fn main() {
    println!("Hello, world!");
    loop {
        print_prompt();
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim() == ".exit" {
            println!("Exiting from process...");
            process::exit(0x0100);
        } else {
            println!("{}", input);
        }
    }
}

fn print_prompt() {
    print!("db > ");
}
