use algexenotation::prime_with_miller_rabin as prime;

fn main() {
    let mut flags = vec![0_u8; 10_000];
    let mut count = 0;
    let mut true_count = 0;
   
    let max = len(&flags); 
    println!("{}", max);
    for i in 3..max {
        if prime(i) {true_count += 1}
        if has(&flags, i) {continue}

        count += 1;

        let mut n = 0;
        loop {
            let j = i.pow(2) + n * i;
            if j >= max {break}

            set(&mut flags, j);
        
            n += 1;
        }
    }

    println!("\n\n{}", count);
    println!("{}", true_count);
}

fn len(flags: &[u8]) -> u64 {
    flags.len() as u64 * 16 + 3
}

fn has(flags: &[u8], ind: u64) -> bool {
    if ind == 2 {return false}
    let ind = if ind > 2 {
        if ind % 2 == 0 {return true}
        (ind - 2) / 2
    } else {return true};
    (flags[(ind / 8) as usize] >> (ind % 8)) & 1 == 1
}

fn set(flags: &mut [u8], ind: u64) {
    if ind == 2 {return}
    let ind = if ind > 2 {
        if ind % 2 == 0 {return}
        (ind - 2) / 2
    } else {return};
    flags[(ind / 8) as usize] |= 1 << (ind % 8)
}

