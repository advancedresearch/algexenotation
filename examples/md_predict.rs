/*

Shows that there is a predictable pattern in `md`.

*/

use algexenotation::*;

fn main() {
    let mut x = 2;
    let n = std::u64::MAX;
    let mut count = 0;
    let mut last = 1;
    loop {
        let mut m = 0;
        let mut y = x;
        while y != 2 {
            y = y / md(y) + 1;
            m += 1;
        }
        if m > count {
            println!("{}' ({})", x, m);
            count = m;
            println!("  {}", x as f64 / last as f64);
            last = x;
            x = x * 2 - 2;
            if x < last {break}
        }
        x += 1;
        if x >= n {break}
    }
}
