/*

Search for patterns in "quad" codes.

A "quad" code is constructed in the following way:

All primes above 3 follows a 2-4-2-4-2-4-... pattern.
This is due to every prime above 3 ending with `1` or `5`
in a base-6 number system.

The primes that are separated by 2 are called "twin primes".
When this pair consists of two primes,
it is assigned the "quad" code `0b11 = 3`.

When only the first number of a twin prime candidate is a prime,
it is assigned the "quad" code `0b01 = 1`.
When only the second number of a twin prime candidate is a prime,
it is assigned the "quad" code `0b10 = 2`.

When none of the numbers in a twin prime candidate is a prime,
it is assigned the "quad" code `0b00 = 0`.

The `0` "quad" code can be used to reason about gaps,
because sequential `0`s means there are no primes.
It is interesting to search for the lowest continuous gap of
N "quad" codes with `0`s.

*/

use algexenotation::*;

fn main() {
    let mut x = 31056082037.max(
        zero_base(next_zero_base() - 1).unwrap()
    );
    let mut y = 0;
    let mut z = 0;
    let mut zeroes = 0;
    let n = next_zero_base();
    let mut found = false;
    for i in 0..100_000_000_000_u64 {
        let c = code(x);
        if i % 1_000_000 == 0 {
            println!("{} {},{}", c, x, x + 2);
        }
        if c == 0 {
            if zeroes == 0 {
                y = x;
            }
            zeroes += 1;
        } else {
            z = x;
            zeroes = 0;
        };
        if zeroes == n {
            found = true;
            println!("found\n{}\n{} => Some({}),", z, n, y);
            break
        };
        x += 6;
    }
    if !found {
        println!("{}", z);
    }
}

pub fn next_zero_base() -> u64 {
    let mut i = 0;
    loop {
        if zero_base(i).is_none() {
            return i;
        }
        i += 1;
    }
}

pub fn zero_base(n: u64) -> Option<u64> {
    match n {
        0 => Some(3),
        1 => Some(119),
        2 => Some(527),
        3 => Some(1133),
        4 => Some(1331),
        5 => Some(1331),
        6 => Some(15689),
        7 => Some(19613),
        8 => Some(19613),
        9 => Some(31403),
        10 => Some(31403),
        11 => Some(31403),
        12 => Some(155927),
        13 => Some(155927),
        14 => Some(360659),
        15 => Some(360659),
        16 => Some(370265),
        17 => Some(370265),
        18 => Some(370265),
        19 => Some(1349537),
        20 => Some(1357205),
        21 => Some(1357205),
        22 => Some(2010737),
        23 => Some(2010737),
        24 => Some(2010737),
        25 => Some(4652357),
        26 => Some(17051711),
        27 => Some(17051711),
        28 => Some(17051711),
        29 => Some(17051711),
        30 => Some(20831327),
        31 => Some(20831327),
        32 => Some(20831327),
        33 => Some(20831327),
        34 => Some(20831327),
        35 => Some(47326697),
        36 => Some(47326697),
        37 => Some(189695663),
        38 => Some(189695663),
        39 => Some(191912789),
        40 => Some(191912789),
        41 => Some(387096137),
        42 => Some(436273013),
        43 => Some(436273013),
        44 => Some(436273013),
        45 => Some(436273013),
        46 => Some(436273013),
        47 => Some(1294268495),
        48 => Some(1453168145),
        49 => Some(2300942555),
        50 => Some(2300942555),
        51 => Some(2300942555),
        52 => Some(2300942555),
        53 => Some(3842610779),
        54 => Some(3842610779),
        55 => Some(3842610779),
        56 => Some(4302407363),
        57 => Some(4302407363),
        58 => Some(4302407363),
        59 => Some(10726904663),
        60 => Some(10726904663),
        61 => Some(10726904663),
        62 => Some(10726904663),
        63 => Some(10726904663),
        64 => Some(22367084963),
        65 => Some(22367084963),
        66 => Some(25056082091),
        67 => Some(25056082091),
        68 => Some(25056082091),
        69 => Some(25056082091),
        70 => Some(25056082091),
        71 => Some(25056082091),
        72 => Some(25056082091),
        73 => Some(25056082091),
        74 => Some(25056082091),
        75 => Some(25056082091),
        76 => Some(42652618349),
        77 => Some(127976334677),
        78 => Some(182226896243),
        _ => None,
    }
}

pub fn code(x: u64) -> u64 {
    let a = prime_with_miller_rabin(x);
    let b = prime_with_miller_rabin(x + 2);
    match (a, b) {
        (false, false) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (true, true) => 3,
    }
}
