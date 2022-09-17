use algexenotation::{pmd, Algexeno};

fn main() {
    let mut x = 0;
    let n = 1000;
    loop {
        let y = pmd(x);
        println!("{}' = {} ::: {}' = {}", x, Algexeno::Orig(x).eval(), y, Algexeno::Orig(y).eval());
        x += 1;
        if x > n {break}
    }
}
