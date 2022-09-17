use algexenotation::{algexeno_mul, Algexeno};

fn main() {
    let mut x = 0;
    loop {
        if algexeno_mul(x) {
            println!("{}' = {}", x, Algexeno::Orig(x).eval());
        }
        x += 1;
    }
}
