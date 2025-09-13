/*
# Minimum Primbix Sequence

This example searches for new numbers in the minimum primbix sequence.

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

/*

*/

fn main() {
    /*
    let tasks: Vec<u64> = (17600..17700).step_by(100).collect();
    for &t in &tasks {
        task(t * 1_000_000_000);
    }
    */
    task(80600 * 1_000_000_000);
}

pub fn task(start: u64) {
    let end = start + 100_000_000_000;
    let mut x = start + 1;

    let mut pr = 0;
    let mut lev = 0;
    let debug = false;
    loop {
        let y = primbix(x);
        if y > 8 {
            let progress = (x - start) as f64 / (end - start) as f64;
            if debug {
                println!("  {} ({}%)", x, (10_000.0 * progress).round() / 100.0);
            }
        }
        if y > lev {
            if debug {println!("{}, {:?}", x, y);}
            lev = y;
            pr = x;
            // if lev == 11 {break}
        }
        x += 2;
        if x >= end {break}
    }
    println!("[{}, {}): {}, {}", start, end, lev, pr);
}

/*
fn main() {
    let start = 664_000_000_000;
    let end =   665_000_000_000;

    let threads = 1;
    let res = min_primbix(start, end, threads);
    println!("\n[{}, {}): {}, {}", start, end, res.0, res.1);
    println!("END");
}

/// Calculates the minimum highest primbix found in range, using a specified number of threads.
///
/// Should be called starting with an even number.
pub fn min_primbix(start: u64, end: u64, threads: u8) -> (u64, u64) {
    let mut s = start;
    let chunk = (end - start) / threads as u64;

    let mut handles = vec![];
    for th in 0..threads {
        let size = if th + 1 == threads {end - s} else {chunk};
        handles.push(std::thread::spawn(move || find_min_primbix(s, s + size, th)));
        s += size;
    }

    let mut m = (0, start);
    for h in handles.into_iter() {
        let res = h.join().unwrap();
        if res.0 > m.0 || (res.0 == m.0 && res.1 < m.1) {m = res}
    }
    m
}

pub fn find_min_primbix(start: u64, end: u64, id: u8) -> (u64, u64) {
    use algexenotation::prime_with_miller_rabin as prime;

    let x2 = (end - 1) >> 1;
    let x3 = (start - 1) >> 1;
    let max_p = (x2 >> 1) + 1;
    let mut min: (u64, u64) = (0, start);
    let mut progress: f64 = 0.0;
    for r in 2..max_p {
        if !prime(r) {continue}

        let mut pr_r = 0;

        for s in x3 / r..r + 1 {
            let y = 1 + 2 * r * s;
            if y < start {continue}
            if y >= end {break}
            if !prime(s) {continue}
            if prime(y) {
                pr_r = if pr_r == 0 {primbix(r)} else {pr_r};

                let cont = if let Some(mpr_s) = max_primbix(s) {
                    let mpr = pr_r + mpr_s;
                    let (min_pr, min_x) = min;
                    mpr > min_pr || mpr >= min_pr && y < min_x
                } else {true};
                if !cont {continue};

                let pr = pr_r + primbix(s);
                let (min_pr, min_x) = min;
                min = if pr > min_pr || pr >= min_pr && y < min_x {
                        println!("{}: {}, {}", id, pr, y);
                        (pr, y)
                    } else {min};
                println!(" {} tried {}, {}", id, pr, y);
            }
        }

        let new_progress = (10000.0 * r as f64 / max_p as f64).floor();
        if new_progress > progress {
            println!("{}: {} %", id, new_progress / 100.0);
            progress = new_progress;
        }
    }
    println!("");
    let (min_pr, min_x) = min;
    println!("[{}, {}): {}, {}", start, end, min_pr, min_x);
    min
}

pub fn max_primbix(x: u64) -> Option<u64> {
    Some(match x {
        0..2 => 0,
        2..13 => 1,
        13..53 => 2,
        53..317 => 3,
        317..3407 => 4,
        3407..8243 => 5,
        8243..49459 => 6,
        49459..197837 => 7,
        197837..1187023 => 8,
        1187023..18314419 => 9,
        18314419..112006043 => 10,
        112006043..491325707 => 11,
        491325707..2047857083 => 12,
        2047857083..9869296583 => 13,
        9869296583..107478087043 => 14,
        107478087043.. => return None,
    })
}
*/
