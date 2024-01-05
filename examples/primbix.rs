/*

2: 1
13: 2
53: 3
317: 4
3407: 5
8243: 6
49459: 7
197837: 8
1187023: 9
18314419: 10
112006043: 11
491325707: 12
2047857083: 13

*/

use algexenotation::prime_with_miller_rabin as prime;

fn main() {
    let n: u64 = 1_000_000_000_000_000;
    let mut depth = 13;
    let mut x = 5827000000;
    loop {
        if x >= n {break}

        let y = primbix(x);
        if y > depth {
            println!("{}: {}", x, y);
            depth = y;

            // I think it is safe to double.
            x *= 2;
        } else if x % 1_000_000 == 0 {
            println!("{}", x);
        }

        x += 1;
    }
}

pub fn primbix(x: u64) -> u64 {
    if !prime(x) {return 0}
    if x == 2 {return 1}
    let x = (x - 1) >> 1;
    for r in 2..(x >> 1) + 1 {
        if x % r != 0 {continue}
        if !prime(r) {return 1}

        let s = x / r;
        return if !prime(s) {1} else  {primbix(r) + primbix(s)}
    }
    1
}
