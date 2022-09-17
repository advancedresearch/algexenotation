use algexenotation::{algexeno_pow, Algexeno};

fn main() {
    let mut x = 0;
    loop {
        if algexeno_pow(x) {
            println!("{}' = {}", x, Algexeno::Orig(x).eval());
        }
        x += 1;
    }
}
