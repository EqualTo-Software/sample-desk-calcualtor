mod evaluate;
mod lexer;
mod parser;
mod tokens;

use crate::evaluate::evaluate;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("Input: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Failed reading command");
        input = input.trim().to_string();

        if input == ".exit" {
            println!("Bye!");
            break;
        }

        match evaluate(&input) {
            Ok(f) => println!("Output: {}", f),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
