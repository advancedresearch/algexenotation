use algexenotation::prime_with_miller_rabin as prime;
use algexenotation::primbix;

/*

0..1 billion

921144 are primbixes of the form `1 + 2*2*s`.
1256368 are primbixes of the form `1 + 2*3*s`.
517843 are primbixes of the form `1 + 2*5*s`.
340137 are primbixes of the form `1 + 2*7*s`.
205593 are primbixes of the form `1 + 2*11*s`.
172946 are primbixes of the form `1 + 2*13*s`.

43259 are primbixes of the form `1 + 2*53*s`.
8085 are primbixes of the form `1 + 2*317*s`.
896 are primbixes of the form `1 + 2*3407*s`.
412 are primbixes of the form `1 + 2*8243*s`.
88 are primbixes of the form `1 + 2*49459*s`.
30 are primbixes of the form `1 + 2*197837*s`.
3 are primbixes of the form `1 + 2*1187023*s`.
1 are primbixes of the form `1 + 2*18314419*s`.
    201458609
0 are primbixes of the form `1 + 2*112006043*s`.

Candidates:

217030341802819
7009359316472207
34283832717323939
71430313138610963
593522650119444443
1098930158356815179
2677000706373711467

2: 1
    29: 2
        3539: 3

        4583: 5
        119159: 7
        14537399: 8
        901318739: 10
        142408360763: 13
        36171723633803: 14
        217030341802819: 15
        // Insert
    1103: 4
    6619: 5
    66191: 6
    21578267: 7
    1855730963: 9
    248667949043: 11
    30337489783247: 12
    4429273508354063: 13
    593522650119444443: 15
    8309317101672222203

    53: 3
        3923: 4

        7103: 5

        13463: 4

        16007: 4
    743: 4
    19319: 6
    2356919: 7
    89562923: 9
    2328635999: 11
    703248071699: 12
    533062038347843: 13
    71430313138610963: 15
    7411029282467616991

    149: 2087
    2087: 3
    154439: 4
    9575219: 6
    363858323: 8
    44390715407: 9
    266344292443: 10
    69782204620067: 13
    17724679973497019: 14
    1098930158356815179: 16
    5729719410140382323
13: 2
    131: 3
        3407: 5
            47699: 6
            1812563: 8
            10875379: 9
            108753791: 10
            22403280947: 12
            313645933259: 13
            94721071844219: 14
            7009359316472207: 15
            602804901216609803
        20443: 6
        204431: 7
        7768379: 9
        574860047: 10
        42539643479: 11
        5189836504439: 12
        633160053541559: 13
        191214336169550819: 14
        2677000706373711467: 15
        1029084878978153843
    787: 4
    4723: 5
    160583: 6
    963499: 7
    9634991: 8
    828609227: 10
    21543839903: 12
    6764765729543: 13
    2448845194094567: 14
    34283832717323939: 15
    7128995133414106879
79: 3
2687: 4
37619
526667
20013347
2921948663
181160817107
59058426376883
1535519085798959
21497267201185427
1590797772887721599
3824424746718550771
16648458306640607563




395151904723: 15
564491890223: 15
1515853827107: 15
1663679308099: 15
1042472702519: 15
1533177778667: 15
1549983656183: 15
2909304357203: 15
4750507979419: 15
5072425583123: 15
5085002594267: 15
8076873750767: 15
9342198250379: 15
16480979201627: 15
17576750259767: 15
24360806609423: 15
26040862237459: 15
47774318125319: 15
1542272186393579: 15
595767937964428343: 15

*/



pub fn choices<const N: usize>(pr: u64) -> [u64; N] {
    let mut res = [0; N];
    
    let mut count = 0;
    let mut i = 3;
    loop {
        if prime(i) {
            let r = 1+2*pr*i;
            if r > i && prime(r) {
                // print!("\t\t{}: {}\n", r, primbix(r));
                if rand::random::<f64>() < 0.5 {
                    res[count] = r;
                    count += 1;
                    if count >= N || r > 100_000_000_000 {break}
                }
            }
        }
        if i > 100_000_000_000 {break}
        if rand::random::<f64>() < 0.009 {break}
        i += 2;
    }
    res
}

// 395151904723: 15
// 176297511683: 15

fn main() {
    let record = 176297511683;
    const N: usize = 200;
    'outer: loop {
    let mut min: (u64, u64) = (2, 1);
    let mut stack: Vec<(u64, u64)> = vec![(2, 1)];
    
    let chars = &[".", ":", "<", ">", "=", "~"];
    while let Some((pr, val)) = {
        if stack.len() == 0 {None}
        else {
            let ind = rand::random_range(0..stack.len());
            Some(stack.swap_remove(ind))
        }
    }
    {
        if pr > 1_000_000_000_000 {continue}
        // println!("stack {} {}: {}", stack.len(), pr, val);
        if val > 15 {
            continue;
        }
        if val > min.1 || val == min.1 && pr < min.0 {
            min = (pr, val);
            if pr < record && val >= 15 {
                println!("\nNEW RECORD!");
                println!("\t{}: {}", pr, val);
                break 'outer;
            }
            // println!("\t{}: {}", pr, val);
        }
        
        stack.extend(choices::<N>(pr).into_iter()
            .filter(|&pr| pr != 0 && rand::random::<f64>() < 0.9 && pr <= record)
            .map(|pr| (pr, primbix(pr))));
        

        if rand::random::<f64>() < 0.002 {break}
    }
    if min.1 == 15 {print!("{}", chars[rand::random_range(0..chars.len())]); use std::io::Write; std::io::stdout().flush();};
    }
}

