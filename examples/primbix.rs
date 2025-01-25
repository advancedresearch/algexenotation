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
    let start = 0;
    let end = start + 10000;
    let threads = 2;
    let res = min_primbix(start, end, threads);
    println!("\n[{}, {}): {}, {}", start, end, res.0, res.1);
    println!("END");
}

/// Calculates the minimum highest primbix found in range, using a specified number of threads.
///
/// Should be called starting with an even number.
pub fn min_primbix(start: u64, end: u64, threads: u8) -> (u64, u64) {
    fn f(start: u64, end: u64, id: u8) -> (u64, u64) {
        println!("{}: [{}, {})", id, start, end);
        let mut m = (0, start);
        for x in start..end {
            if x % 2 == 0 {continue}
            if x % 3 == 0 {continue}
            if x % 5 == 0 {continue}

            let y = primbix(x);
            if y > m.0 {
                println!("{}: {}, {}", id, y, x);
                m = (y, x)
            }
        }
        m
    }

    let mut s = start;
    let chunk = (end - start) / threads as u64;

    let mut handles = vec![];
    for th in 0..threads {
        let size = if th + 1 == threads {end - s} else {chunk};
        handles.push(std::thread::spawn(move || f(s, s + size, th)));
        s += size;
    }

    let mut m = (0, start);
    for h in handles.into_iter() {
        let res = h.join().unwrap();
        if res.0 > m.0 || (res.0 == m.0 && res.1 < m.1) {m = res}
    }
    m
}
