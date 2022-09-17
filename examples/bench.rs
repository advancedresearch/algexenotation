use algexenotation::Algexeno;

fn main() {
    let mut x = 0;
    let n = 100_000;
    let mut c = 0;
    loop {
        let y = Algexeno::Orig(x).eval();
        if let Algexeno::Const(_) = y {
            c += 1;
        }
        x += 1;
        if x > n {break}
    }
    println!("{}", c);
}
