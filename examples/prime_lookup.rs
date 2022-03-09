use algexenotation::{prime, last_in_prime_lookup};

fn main() {
    let mut n = last_in_prime_lookup().0;
    for _ in 0..8 {
        loop {
            n += 1;
            if prime(n) {
                print!("{},", n);
                break;
            }
        }
    }
    println!("");
}
