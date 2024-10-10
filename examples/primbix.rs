/*
# Minimum Primbix Sequence

This example searches for new numbers in the minimum primbix sequence.

Uses a single thread.

*/

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
9869296583: 14

*/

use algexenotation::primbix;

fn main() {
    let n: u64 = 1_000_000_000_000_000;
    let mut depth = 14;
    let mut x = 25_000_000_000;
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
