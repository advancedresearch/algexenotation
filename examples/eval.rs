/*

# Alegenotation Eval

Evaluates (converts) from original form to Algexenotation.

*/

use algexenotation::*;

fn main() {
    println!("=== Algexenotation Eval ===");
    println!("Type number to evaluate to Algexenotation.");
    println!("Type `bye` to quit.");
    loop {
        use std::io::{self, Write};

        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("ERROR: Could not read input");
                continue;
            }
        };

        match input.trim() {
            "bye" => break,
            x => {
                let x = match u64::from_str_radix(x, 10) {
                    Ok(x) => x,
                    Err(_) => {
                        println!("ERROR: Could not convert input to u64");
                        continue;
                    }
                };
                println!("{}", Algexeno::Orig(x).eval());
            }
        }
    }
}
