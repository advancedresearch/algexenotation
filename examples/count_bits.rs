use algexenotation::*;

fn main() {
    let mut cache: Vec<(u64, u64)> = vec![
    (
        2,
        2,
    ),
    (
        3,
        65537,
    ),
    (
        7,
        550829555713,
    ),
    (
        23,
        687731638273,
    ),
    (
        31,
        756451115009,
    ),
    (
        311,
        756453212161,
    ),
    (
        127,
        757524889601,
    ),
    (
        383,
        757793292353,
    ),
    (
        991,
        757826846737,
    ),
    (
        2039,
        757893959681,
    ),
    (
        3583,
        757912829953,
    ),
    (
        6143,
        757914927233,
    ),
    (
        8191,
        757923315841,
    ),
    (
        73727,
        757925412869,
    ),
    (
        63487,
        757925937157,
    ),
    (
        129023,
        757925970433,
    ),
    (
        131071,
        757925970947,
    ),
    (
        522239,
        757925972497,
    ),
    (
        524287,
        757925972357,
    ),
    (
        1966079,
        757925972389,
    ),
    (
        4128767,
        757925972437,
    ),
    (
        16250879,
        757925972413,
    ),
    (
        14680063,
        757925972477,
    ),
    (
        33546239,
        757925969873,
    ),
    (
        67108351,
        757925969893,
    ),
    (
        201064447,
        757925969599,
    ),
    (
        260046847,
        757925969887,
    ),
    (
        536739839,
        757925937107,
    ),
    (
        1073479679,
        757925937127,
    ),
    (
        5335154687,
        757925928959,
    ),
    (
        2147483647,
        757925412859,
    ),
    (
        8581545983,
        757925412863,
    ),
    (
        16911433727,
        755914243921,
    ),
    (
        32212254719,
        755914244017,
    ),
    (
        51539607551,
        755914243957,
    ),
    (
        206141652991,
        755914244023,
    ),
    (
        266287972351,
        755914244087,
    ),
];

    let mut changes = 0;
    let start: u64 = cache.iter().map(|n| n.1).max().unwrap_or(0);
    for n in start..1_000_000_000_000_u64 {
        if prime_with_miller_rabin(n) {
            changes += 1;
            let bits = n.count_ones() - 1;
            if bits < cache.len() as u32 {
                if cache[bits as usize].0 == 0 {
                    cache[bits as usize].0 = n;
                }
                cache[bits as usize].1 = n
            } else {
                for _ in cache.len() as u32..bits {
                    cache.push((0, 0));
                }
                assert_eq!(bits, cache.len() as u32);
                cache.push((n, n));
            }
            if changes % 1_000_000 == 0 {
                println!("{:#?}", cache);
            }
        }
    }
}
