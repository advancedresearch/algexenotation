use algexenotation::prime_with_miller_rabin as prime;
use algexenotation::nth_prime_with_lookup as nth_prime;

fn main() {
    let mut x = 3;
    let mut primes = vec![2];
    primes.push(x);

    // let mut candidates = vec![];
    let stop = 38;
    loop {
        let n = x + 6 - (x % 6);
        let p = gap(n);

        let mut candidates = 0;
        for y in (x+1).max(n+1-p)..n+p {
            let simple =
            y % 2 == 0 ||
            y % 3 == 0 ||
            y % 5 == 0 ||
            y % 7 == 0 ||
            y % 11 == 0;
            if simple && y <= 11 ||
            !simple && y > 11 {
                candidates += 1;
                primes.push(y)
            } else {
                let simple =
                y % 2 == 0 ||
                y % 3 == 0 ||
                y % 5 == 0 ||
                y % 7 == 0 ||
                y % 11 == 0;
                if !simple {
                    println!("false");
                    return;
                }
            }
        }
        primes.sort();
        primes.dedup();
        /*
        if candidates <= 2 {
            println!("{}", x);
        }
        */

        /*
        candidates.clear();
        for y in (x+1).max(n+1-p)..n+p {
            candidates.push(y);
        }
        for &pr in &primes {
            // println!("{} {:?}", pr, candidates);
            for i in (0..candidates.len()).rev() {
                if candidates[i] % pr == 0 {
                    candidates.swap_remove(i);
                }
            }
            if candidates.len() == 2 {
                println!("{}", x);
                break
            };
        }
        for &ca in &candidates {
            if ca != x {
                // println!("{}", ca);
                primes.push(ca);
                // x = x.max(ca);
            }
        }
        */

        // x = n+p;
        x += 1;

        // x = *primes.last().unwrap();
        if primes.len() >= stop {break}
    }
    println!("len {}", primes.len());
    primes.sort();
    primes.dedup();
    println!("{:?}", primes);
    println!("len {}", primes.len());

    let check = false;
    if check {
        for (i, &p) in primes.iter().enumerate() {
            if !nth_prime(i as u64) == p {
                println!("wrong {}th prime {}, expected {}", i, p, nth_prime(i as u64));
                return;
            }
            if !prime(p) {println!("{}", p)}
            println!("checked {}th prime {}", i, p);
        }
    }
}

pub fn gap(n: u64) -> u64 {
    let a = (n as f64 / 4.8 + 1.0) as u64;
    let b = (4.55016035513408 * (n as f64).sqrt()) as u64;
    a.min(b)
}
