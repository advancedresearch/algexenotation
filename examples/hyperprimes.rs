use algexenotation::*;

use std::io::Write;

fn main() {
    for i in 0..10 {
        print!("{}, ", ax!(i).original());
        std::io::stdout().flush().unwrap();
    }
    println!("");
}
