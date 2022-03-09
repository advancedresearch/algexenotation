use algexenotation::*;

fn main() {
    let a = 0;
    let b = 2;
    let c = 4;
    let d = 6;
    let g = ax!(
        (1 + (1+a*b)*c) *
        (1 + c*d)
    );
    // prints `1737040751'`.
    println!("{}", g.original());
}
