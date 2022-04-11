/*

Search for Lonely Numbers with gap corresponding to Hamming distance 1.
See paper https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/natural-loneliness.pdf

*/

use algexenotation::*;

fn main() {
    let n = last_lonely_lookup_index() + 1;
    println!("{} => {},\n", n, lonely_with_lookup(n));
}

pub fn lonely_lookup(n: u64) -> Option<u64> {
    Some(match n {
        0 => 2,
        1 => 5,
        2 => 11,
        3 => 11,
        4 => 37,
        5 => 107,
        6 => 107,
        7 => 127,
        8 => 127,
        9 => 173,
        10 => 173,
        11 => 173,
        12 => 251,
        13 => 251,
        14 => 373,
        15 => 373,
        16 => 373,
        17 => 373,
        18 => 373,
        19 => 373,
        20 => 373,
        21 => 373,
        22 => 373,
        23 => 1553,
        24 => 1553,
        25 => 1553,
        26 => 1553,
        27 => 1553,
        28 => 1553,
        29 => 1553,
        30 => 1553,
        31 => 1553,
        32 => 2371,
        33 => 2999,
        34 => 2999,
        35 => 2999,
        36 => 2999,
        37 => 2999,
        38 => 2999,
        39 => 2999,
        40 => 3181,
        41 => 3181,
        42 => 3181,
        43 => 3181,
        44 => 3181,
        45 => 3181,
        46 => 3181,
        47 => 3181,
        48 => 3181,
        49 => 3181,
        50 => 3181,
        51 => 3181,
        52 => 3181,
        53 => 3181,
        54 => 3181,
        55 => 3181,
        56 => 3181,
        57 => 3181,
        58 => 3181,
        59 => 3181,
        60 => 3181,
        61 => 6323,
        62 => 6323,
        63 => 6323,
        _ => return None,
    })
}

/// Gets the index of the last lonely prime number in lookup table.
pub fn last_lonely_lookup_index() -> u64 {
    let mut x = 0;
    loop {
        let y = lonely_lookup(x);
        if y.is_none() {
            return x - 1;
        }
        x += 1;
    }
}

/// Returns the first prime that has no neighbor primes
/// up to a mask of `n` bits.
pub fn lonely_with_lookup(n: u64) -> u64 {
    if let Some(x) = lonely_lookup(n) {
        return x;
    } else {
        let mut x = last_lonely_lookup_index();
        let y = lonely_lookup(x).unwrap();
        if !prime_with_miller_rabin(y ^ (1 << (n - 1))) {
            return y;
        }
        x = count_primes_with_lookup_and_miller_rabin(y) + 1;
        loop {
            let y = nth_prime_with_lookup_and_miller_rabin(x);
            if lbit(y, n).is_none() {
                return y;
            }
            x += 1;
        }
    }
}
