/*

# Algexenotation Type Clock

This shows the sequence of algexenotation
of some Algexenic Type (there are 11 possible types 0-10),
where the time elapsed is equal to `x` in milliseconds.

*/

use algexenotation::*;

use Algexeno::*;

fn main() {
    use std::time::Duration;

    let search_ty = 3;
    let mut time = 0;
    let interval = 1_000;
    for x in 0..1_000_000_000_000 {
        if x > time && (x - time) % interval == 0 {
            std::thread::sleep(Duration::new(1, 0));
            time += interval;
        }
        let y = Orig(x).eval();
        if y.ty() == search_ty {
            println!("{} {}", x, y);
        }
    }
}
