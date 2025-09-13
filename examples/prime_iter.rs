use algexenotation::prime_with_miller_rabin as prime;

fn main() {
    let trad = false;
    // let end = 100_000_000;
    let end = 100_000;
    let start = 30_000_000_000;
    if trad {
        let mut n = 0;
        for i in start..start + end {
            if prime(i) {
                // print!("{} ", i);
                n += 1
            }
        }
        println!("{}", n);
    } else {
        let mut n = 0;
        for i in (PrimeIter {ind: start, end: start + end}) {
            n += 1;
            // print!("{} ", i);
            /*
            if !prime(i) {
                println!("Non-prime {}", i);
                break
            }
            */
            /*
            if n % 10_000 == 0 {
                println!("{}", i);
            }
            */
        }
        println!("{}", n);

        /*
        let mut n = 0;
        let mut full = 0;
        let mut full_m1 = 0;
        let mut full_m2 = 0;
        let mut full_m3 = 0;
        let mut full_m4 = 0;
        let mut full_m5 = 0;
        let mut full_m6 = 0;
        let start = 0;
        let iter = PrimeIter {ind: start, end: start + end};
        for buf in (Pack {iter, f: prime}) {
            match buf.count_ones() {
                64 => full += 1,
                63 => full_m1 += 1,
                62 => full_m2 += 1,
                61 => full_m3 += 1,
                60 => full_m4 += 1,
                59 => full_m5 += 1,
                58 => full_m6 += 1,
                _ => {}
            }
            n += 1; // buf.count_ones();
            // println!("{:064b}", buf);
        }
        println!("end iter\n");
        println!("{}", n);
        println!("full {}", full);
        println!("full_m1 {}", full_m1);
        println!("full_m2 {}", full_m2);
        println!("full_m3 {}", full_m3);
        println!("full_m4 {}", full_m4);
        println!("full_m5 {}", full_m5);
        println!("full_m6 {}", full_m6);
        println!("{}", full as f64 / n as f64);
        println!("{} KB/billion", (n - full) as f64 * 8.0 / 1024.0 * 100.0);
        */
    }

    /*
    // Gen
    let start = 52069;
    let mut n = 0;
    for i in (PrimeIter {ind: start+1, end: 65536}) {
        if prime(i) {print!("{}, ", i)};
        n += 1;
        if n > 500 {break};
    }
    println!("");
    */

    /*
    let mut n = 0;
    let mut prev = 257;
    for i in 258..316228 {
        if prime(i) {
            print!("{},", (i-prev)/2);
            prev = i;
            n += 1;
            if n % 40 == 0 {println!("")}
        }
    }
    println!("");
    */
}

pub struct Pack<I: Iterator> {
    pub iter: I,
    pub f: fn(I::Item) -> bool,
}

impl<I: Iterator> Iterator for Pack<I> {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: u64 = 0;
        for p in (0..64).rev() {
            if let Some(n) = self.iter.next() {if (self.f)(n) {buf |= 1 << p}}
            else if p == 63 {return None}
            else {return Some(buf)}
        }
        Some(buf)
    }
}

static GAP: &[u64] = &[
0xfe770238d388fe7e,0xfe3c104efe38ce3c,0x7ccae3f9c7f3f708,0x8ff85fc1833f79de,0x340608e0471a38d6,
0xf063fd1ff9cbf04e,0x209e9c7fe7fbc183,0xfc2ff9c8a8fcee33,0x3a8ff28238fe39c8,0x8e3e4e1efe3c68fc,
0x7e70bee59c060cff,0xd770839a4ee380ff,0x53537f9dc79c4083,0x98e03353b832acfe,0x6a3cd479fb337f3b,
0xe9dcef9fd1f39ded,0xa18fcfb99ac65047,0x9fb8fe39c418d019,0xdcd23f9dd60ff014,0x7fc560c68f0f2fac,
0x4d6d70f67e3cde38,0x559ff7e16609a9ae,0xbd638e6f8788fce2,0x110277f3fd18d3bc,0x19fefe047ea04702,
0x3ca7e3f0b5ee63fe,0x7de7d7339d47d1ca,0xb0f04107820f9788,0x338ae4caeffa813b,0x8ed0fd67f5a8fcfc,
0x726a7759671f85a5,0xe394e0c7f388d3f7,0xe09a394f99ff57a3,0xc1869c763e6bf408,0xefffe770f73efe3a,
0x8ccef6fef06c6a77,0x1c09de0eb73def07,0xf9cbf3bf04d75b28,0xaf5bacefa7e20382,0x0c22d1e77943f9c1,
0x83ff9c79ff196fbc,0xa782fca7079dd61a,0xd787882708ed3504,0x0b71e6bf8758fdf0,0x4effcecfc2d08b73,
0xe6f943a39c677ef4,0x1089a956afaaf29f,0x8307f8fc10bfe169,0x4fade778d3b42e5e,0xedf929dc75ce7661,
0xbf53ee9c1f1e0fec,0x9de1a1047d7da38d,0x39b0f825efc2d3a8,0x39c46b19a811c3f5,0xf1e3f9f9effca6b6,
0x60f018d7efef01f1,0x59dcd19c658faed3,0xb39cbe799a5b82b1,0xa39f7e9f8f047378,0xff3d01f8d4a1efc3,
0xcea0cde1efd43d9c,0xe5f3e12d3ebc195d,0x44566fe033bcdfc1,0x8fe33456ffc73460,0xc11f43be2b1adc68,
0x0c3a8ae6f8ff4787,0xf6b750f7d2860d8e,0x3f9f5e1eac19af9c,0x5962085f5b868411,0xaf5b5e767e13de6f,
0x3483929dad9f3fd6,0x6f15ced6cd96f04d,0x425447cff9dcf66b,0x1fa3fe1e20eb7b5c,0xfbcb3f1fb7c7147c,
0xd1820f8f0efd1ed7,0x08c785a7d7c7fe70,0x67facfc89e9ffcd6,0x7194d3d0c067254b,0xf8fc747e7eea3967,
0xfa5fdf3d6f1e7eef,0x80e0cb72d1f8aa6a,0x1ed63ff80a04d204,0xfaf9f4b13b8cc13e,0xfe3ac19bc148e3a8,
0xfe097bbe094eba08,0xd6a7be8d6137f2f2,0x2a34fbd43de3ce21,0xa09a47fcee2b15f1,0xf465348fe77ffc78,
0xd359d9cd418fede3,0x1f297b4df0bf83a0,0xf0129fd0f94fce20,0xcfc4a10bf80a1783,0xf5b4d20c065720c7,
0x0f78f6671c07f47f,0xfcea69f87fed3732,0x87db671c68ffcd19,0x74e3b7f29a756938,0x339bfbfd1cb3ff68,
0xaf9fdec99a9d9a9d,0xed3cbb58f855c7cc,0xaeff5ea7f43c79fd,0xb5f074df35e1737c,0xa513d3ff065771e6,
0xa13e9ffe721669f9,0xa811df0196fac10e,0xfe69621340f82715,0x0f7e5958e09a39bc,0xee0eb230ffdd466a,
0x1ee6894794ca7ef3,0xb5908e6a1bdfb2ec,0x3083c9f9a4efa958,0xbcc71529f7c75d29,0xf9d454d7a96739e4,
0x67d1c0977e7651c7,0x1e71fcff9f61073f,0xa7f72d7c418099da,0x6b0fde0cfbcdfe02,0x60e63413d33947f7,
0xfa384bf54668e50d,0xafe7ee05bf8d6228,0x8ed091e1bf9d19bd,0x1410971f33552ef3,0x5f0475830ba94238,
0xf159f73d29fcfc63,0x943af7cf7b8ebdf1,0xe77ca284ee768e75,0x63f820823be1228a,0x0878e85a3a831e03,
0x559eb49f847e12ff,0x3350f69dc34d4d13,0x1c785a9f508e6d9f,0xe8ca0b13d3977e6a,0x3ca07c18f982b3b5,
0xbea7c1edd35ea058,0xcefc10b8f0734875,0x9f58e7ab5031a0ff,0xd18ad268a58efe0c,0x71a1efa38eff3897,
0xef1fcff44fc72c5f,0xcfc4e554ece72fa7,0x23fcfdc785a509ae,0xbe31dea4e6e673af,0xe53daf2860e8d7bd,
0x1c54ff9cdb48d0f0,0x57de51a11ac23f05,0xaa35e90277d7c787,0xde33bf9771c34fe0,0xf550eb3bfe5cb46b,
0xd47cce4f683f409a,0xce5e6fe56fe57d20,0xe67794751a3c26fd,0x639f7383fe9e8182,0x3dace3ac09aff07c,
0xe42c633b699af7fc,0xfb98ff823fe7debd,0x1c3c19a820aeb73e,0xffd743a9fb42e569,0x49e8ac3cf7357e5b,
0xde253e7379f7a820,0xbe2bd35d7a9dec38,0xfc2ff33bf9dadec0,0xcd4de3f9f48eb1af,0x93c410aa6a50c1b2,
0xc7f0417c53c794fe,0x6935d1adf80ce7f6,0x6767e39e87c3f79c,0xd34ed7f58f37c3fe,0x8d5f8e3fb5a5b7f3,
0x8e9f91e1072a87cd,0x7f9a5b8203f3858a,0xdafdf1e7269e6fe1,0x51e8f8aa691a17fc,0x52facdbef3544d4f,
0xfbe8e6f878f868f1,0x374fdd75ef0d9d54,0xde33842395ab8ff9,0x1ce77d2fe337f844,0x13880d2fbcdffbfa,
0x727882f9ac73d39d,0xed79c47f1fbccfda,0xf7c71dca78f378d0,0x46be2baf5f3b59de,0x38454dfd5f020cd6,
0xcfe301f537f1fcfc,0x54de6907842da76a,0x63e6343df9dac41c,0xcb13fbb4769dfcd7,0x80a751d3b97ef8fe,
0x38acef6d6f3ebcd1,0x85fe83e175066bfb,0xe0a831e1b5dc2398,0x5c7e90f697bc19c5,0x634d7c7e085c7f1f,
0xd8b0ca1388eb99a9,0xbff94e8e76bf7ee6,0x7a772ae1e212ac69,0xbf9f84707c083c3b,0xd60a30b304f5fa8f,
0x0bf5535fe7158304,0x75b4ef1cdff4d758,0x32ae9da706718f93,0x8f9bd7edc1817fb2,0x828e72d0437ea1ee,
0x3fb5d47d364f72e6,0xbf3b5cfbf84f1509,0x147e71f1c6bb7e10,0x4dfa0daeb0907e6b,0xe7fbc0b1f66ae23e,
0x8f6b8463c2ce6f9c,0x57f9cbca0799dcd1,0x2813830f7e17b7d7,0x1e50c7b42fe96460,0x9aeb0b77c79a94e7,
0x1452be97de77ce08,0x7b9c4d141f3ab3d5,0xea268cad7c18306c,0xdfb5b8d15cfa4bfd,0xed07f8e8f9b9ce5a,
0x3f1d2feae351f977,0x748fcb0f2819a9f5,0xf1c1f04ad61a3c8f,0xe8a3817cfd6d751e,0xc0ceff4f4e50c182,
0x3f05d617ea6b1c08,0x42de0b87e0583ced,0x2f15c796e33e8e03,0x9bf8271fe768ae6a,0xb1f5baf2d6fc9737,
0x83e16baecdbfd933,0xbdaa5c7b497fe110,0xeabe73426c8e0bce,0x3cd21ef9f1a169ed,0xf82575feb1f5bd86,
0x8273a8f9833b813f,0x34a307b780cbc213,0xafe53f7a3fe77d3b,0xc07ec0cde0f6c77f,0x8759dfcd27ee0334,
0x8e35d0839bf2889f,0xb26335683e6b3909,0xcfee59fcee3e8f05,0xac673bdf3d0ff1ce,0x307cd74eed821ed4,
0xcdfe719c841e8c2f,0x684d56513f496073,0x7f05c9e3cd20f6e6,0xad2f70f11a7d419b,0xafbd8f4e38f2e5a6,
0xf3758c13b47d6fc2,0x79d4231f8a9c0634,0x573b8fc3737a09d5,0xd877e77c98ff4d65,0xee0cefed660a26a3,
0x3842de68e7d6029d,0xe16bf779aea677e6,0xf5075285d6341063,0x95a2813fba1fac61,0x66a04a159da717a7,
0xecdc38fe11ebecfe,0xf9c1b1f780d4eefc,0xfe8465632c27dfca,0x065f4559df4e4633,0xb467d4149df4d4ec,
0xadadf314bf04df38,0x2692ca5aec3c57d6,0x3f9dc2ad26928677,0xfe6e60af28422b63,0xbff3f0be4604e11c,
0xdea08d6e2887882b,0xa13be7d7c67e353f,0xaedcd0836fd6776a,0xc245196668a08ec5,0x079dfdaff9be53ac,
0x7684b972e577cce4,0x63961043eaefaf50,0xf063e84eff1cd519,0xf5e766fcd4a169ec,0xe6f3539758d1d5bc,
0xe20b8e13fb38749a,0x338dae1ece115d15,0x9c468a27de047358,0x3e0c7e6bfabd3bbe,0x046b18f29a776fea,
0xa29e10bf2fee7556,0xd66daa29d9d83c60,0xf5349f7e653cbcb3,0x05c6f1a73671a113,0x5d6d7b98e1dedfce,
0xfc04f9f59dd128cf,0xcf5c14ef37f15a94,0x3a081389bfcfa39b,0xc6947db8877aebe1,0x1ce7742fe6f01cbb,
0x9c63ba5bd8239306,0x778f6d3c57d7307e,0x80a5bb5ef29f3354,0x9fbf94d2f75823c2,0xcd4226b1df3ab04a,
0x08f6fa6bef299db6,0x6bde56f199ff7d1d,0x8fa85547f37e6b1a,0xfbf2bb2715f82588,0xb6059e4b43651583,
0xd63cd277ed3bf35f,0xaece34753968e56b,0xf9a5ff7bcf6d7f86,0xf7558f689be04eeb,0x0ab3b3edf02f597f,
0xee04e3637a8e3cdf,0x28ab3469051fb013,0x8827de3f83a766fb,0x7f9dc7f3fdeb1cae,0xe6a963fbe157ff08,
0xcaa79422b023979d,0xf68ce3349f84ee03,0x38c706ddfc34f6bc,0x1d04ae17fbb479f4,0x9aa8cc1391fafbf7,
0x0f94670138fa0e60,0xa943042d159a95c7,0xcd25e17c5a2b0ebe,0x2978d08c2fe3e9be,0x770758cff7a9aeb7,
0xb5a6e8cfc7fed12a,0x9cd4a7f1f33f56e1,0xe6b6b80c19c18259,0xdfb7faff782b6bac,0xfcebe6be6a5d5cdc,
0x7e3cd75b279827de,0x0f9c27eeb0669041,0x0f6b1972f078307f,0x826f1f472c5eda7d,0x2fefcdf1f9bfcd74,
0x7eb0f36bf58a337e,0x82919c6a7f8235fe,0x0f9779d5f0d3b27e,0x161349af8a593cec,0xf3dff9c412826b3b,
0x5b83a50b815042e3,0x4e32efad9cea337c,0x226ae8e6f9be7513,0xb4fa9bfcb6fe17d1,0xe59ff375f68ffce3,
0xe7d27fbfb6c7af43,0x2fafa3ccece7df3b,0x414b13b8ad41ff4b,0xbb4785adec56ba9c,0xefa3fb784a1fa0fd,
0x3d9a6f8461a15961,0x8edebc17c11cbb8a,0xe7f5c23abea1bfde,0xe8c69c7ce0ce44a6,0xb7d3fdffcaa104ef,
0x9a831d4ff8c1e8e1,0xefa6a17ee3fd2d55,0x8fd1ed6829158d35,0x15f9d463a9acf779,0x4efe65fc25da0846,
0x3cfd58ca6b2fd541,0x5f61d78454e1295f,0x9a8d6328418f9eb7,0x3b58db4257d23dad,0x014fda097d6f351e,
0x7fa9c8c2d1f5833f,0x704bf4574fc79a4a,0xdfcdffa814e3eb67,0xa38adb1f37d6c6f8,0xa7b30beb1559ddb0,
0x27787535f34505e8,0xce1e215792b1027f,0x7bc2e047b688f8cf,0xb5608593e6f1c3a4,0x1355b0faf7c1078d,
0x3d9e3e4e70704079,0x2b5561060f1a7e79,0x08135836b9da3e73,0xdf479d1315e814c7,0xf0ef52fadd742e3b,
0x967fc1d063b26937,0xeb08c99acfe9f4df,0xcbad41e3f817abf0,0x6097fea466bc2ac7,0xe8fc11ecf6b77e3e,
0x659e577f0472ee94,0xdf7e6939566f19bc,0xf8d04adcd415b1dc,0xe16472b3dcd4d747,0x353ebe3cdeb0bcf6,
0xdfc37f07cdfc3f7c,0x3bc1e8ce6bb5ef86,0xda417d4ed0420f9a,0x33910784e960e5e1,0x1fced756671cb7b0,
0x8fcffbb406719c54,0xed76061f66a5359b,0xfc73eebd1a875f9e,0x6723359f585feb25,0xf5b304f7efe3b811,
0xcde6a70bf6650bb8,0x17f84bf7a3ca52fa,0xdc227079cfecef5f,0x5221ef3b39cbe8d6,0x5fa8ea68dffe5de6,
0xcbf5487d3589c65e,0xcaf9fd47f5f01f82,0x3b7cdfc3c1f6e854,0xbb4183a0d8780eda,0x7d6d7f5e5e789c1c,
0x7383b77ced3fbdf4,0x7159f7c75550f6b1,0xfa030dea0e7704d1,0x832add9478d1febe,0xf2ae5c671dc3db6d,
0x1f09d4fd94d61ad7,0xc1d01823b7bab60f,0xfc2d3f57c04f5be8,0xe106df3f762bd25f,0x5ded2fcee2b97706,
0x35fa11ecde8f7e6f,0xd2f16833552b7c1f,0xa3ca72aabbb54d7f,0x4bd2da76ba138b41,0x6fedcc54a7e7ee4e,
0x29651ac1b47cc701,0xdbf58f067510eade,0x094f3ebcffb46bed,0x7cefe033bc2d0846,0x76847b7deeab3eb0,
0x20c0bdd7a95dfe60,0xdaf2abd77edd6a36,0x88a9c067de1d4fc2,0x697ddc71f35753bf,0xfa691ac47940a982,
0xeb1aedacdfe39f7f,0xf158293bac08ce02,0x8e12fd5e6e391281,0xc79dcb1a71cb9a72,0xc23fcc69fe8c7b0e,
0xdfe09bd15fbddfe3,0x4ed35089c91f37c2,0x7ebd2f7a9bcd4dd0,0x2c6c3c1e6fccee34,0x3610ea3e9c19a450,
0x67fdac6d25c78239,0xbfe8e8ede3351ac7,0xba83e58f7cc570d5,0x7b9f07473bf75830,0x4a7c98d609c7b6b7,
0xcfa4d4a13f08e5ee,0x3c1e1d7d9d108c3d,0xa08f3b5f3bfd8101,0x4eaa9fe17fe6f284,0xed631f4af75bc715,
0x8ebfe65829f08db6,0xad7e962f3979a5dd,0xed66770208f69566,0xb6f138f9ff07aeed,0x6e68f38ea11b4ea5,
0x58b4dd610ba380fb,0xd1dbdffcfbec77fa,0xf5061138cd29cee2,0xb1530741ffced29f,0x30565f7729f4aecf,
0x3d06dfa9abe0975d,0x54e21a3c12bee0e4,0xa151e11f3b371c54,0x56737ea0c7355b5f,0xcadfadf33975e6e7,
0x3eaaf36827efa94f,0xbcb9b8789779d58d,0x3fa18d1e5efe7f4e,0x3434233060414738,0x419bcfb9f8559b29,
0x758102337dbe779b,0x2380e026a59e6e56,0xcc3a8acf61158d6c,0xf47d37b2da7e2fc1,0x53fd4fdcb3961e09,
0xf7c79dd4a79adafe,0x6ebd63f6e8750b41,0x19ad80963778a0a3,0xa8a95f12ee90b53e,0x6fd05aea6a28cee3,
0x4bbc3ed4fcee3961,0x9f84faa4ab3ff1ac,0x730b4ffb4d294ed1,0xfb635842a307fd1a,0x1e2901f3f563b78f,
0x35d6e0832e65f0b4,0x221e2ade0cfa5b4f,0xcf7e3cdfe39aa4fb,0xa047e340735832d7,0x7ccf4e3be5ef2897,
0x1a29d54239a95fef,0x571c5419f564efa6,0xadce10973cd4dfc1,0x04ed63df9fbbf3ff,0xba98f3f6842dc77f,
0x4d1be3f978c51473,0x75b40bee047b7a93,0x4f3bf15fed836fb3,0xcb05b81353f7f295,0x4fa4effc71987956,
0x7211ecdc426f6670,0x6083e0e9ace44184,0x5638f6fdfa0f15f9,0x73ff5fe71539a186,0xf1047ea3597f147f,
0x638a753b8fafd9c7,0x4bfbe5cf534bde5b,0x79ab2f51e0811e53,0xc6825b59f4b13bbe,0x6b63916c35a87b3e,
0x2b537ffcfa4f7ea0,0x8cc2138c5629fa46,0xbb778eb1a3f2da47,0xb7506c77cf5385c7,0x9dac8b184733d3ff,
0xe5b837a0c3c195de,0x3b2de03875d1e3ad,0x9ffbf9c6d1e083dc,0x17817ffa10ea12f6,0x8ebabb380ca220da,
0xf9c77f9f56b5139d,0xe0ced636bb786eab,0xbdbd5f067e304de3,0x5bea51af7e9c638d,0x29466e82de09c6da,
0x156a96bb8fce36f2,0x7b5301b4783c768d,0x4cdd2d575c4b68c7,0x0418d4e335fa6f3d,0xb8b2f7e8a55f9edc,
0xab29c6bdf04a7810,0x7cfce8a0a82d6e66,0xfd7bc66753f7f153,0x51a7704d419c51e8,0xfd1dafa706382d83,
0x6060f28ab7ca7d3b,0xcdf2fa1b5cdf1e77,0xb595d722a3f0abd7,0xf59fbeb3dafebec0,0xa8756f85475ef349,
0xda7b5ef0fd2c239a,0x3067bc84183c6820,0xd8a78aadafbbf3ef,0x4603ebec9d1fe96e,0x41f63594e39be084,
0x2770dcffcfef19cb,0x4fa381743f30238e,0xc729e6908a394d1f,0x9bd60dae5bbe76bc,0x15d0187bc3a83cde,
0x34a0563c685e09d4,0x79a39be5e259a05e,0xa358f8f1dba3d87d,0x3b9bdc93d9a51a3d,0xdf35374d5753ea0c,
0x73daaffd427b9734,0x53597b9c18af42ad,0x2de7efe6fed9c9e5,0xaee7a412ce5b741f,0x60b8208ff253f565,
0x7eed09eed69ddc70,0x616bdcc1eae8ce77,0x41fc148f6e8e5383,0x3bffa6a1e0cb3a94,0x0afe93f3a2b79d46,
0xd17fb7da750183e5,0xd3ceea7fc63f79ab,0x63f823cd7b8ffded,0x38c1b7d0f3652a30,0xeb3e9023fd35ccd9,
0x35ccea2bcaee5a8b,0x87f660acefd74e3e,0x08a12fee6caace0e,0x39c9eeb9c73ef1a6,0xf7f9dfdad3471cd2,
0x2a56cce44a68a0a3,0xcad653f7a6b1f5dc,0xcb1a751364aa7caf,0xdb5fa39bf42d1488,0xd07cd75b40b158e0,
0x46ba1529e6f58e0c,0xfc26b351e5d33537,0x3954e288d0bc2d35,0xb37fd5be5123379d,0xd753f1c7d07e8e09,
0xdac1b1416f80f9c8,0x8ac156435c60dbd9,0xc7f68c13506d129f,0x597f0895bc1b3358,0xb8684411c0833f1e,
0xbd6dbfe8e3e8a46c,0x6f705a0c78162697,0x721eeaa112dc2771,0xe71fa73ddf7d4a75,0xeffe2a9a9431c7fe,
0x3ac3770f770789f5,0xee1af8961e5fae8f,0xed534f1d96e324fb,0x867928cfcbec256b,0xf8337f1a0fe6ebf7,
0xbe8d7b9bfca1f060,0x41559c23f1e509fd,0x9f80c51f6dfbafa9,0xd18ff32b74f67b3f,0x50e8e58eaad8fce4,
0xe8d2771980d4eebf,0xc2e3f3b8cada7b9f,0x2ee1d4782285e937,0xc66d285af961b544,0x79426a3422becb8f,
0x9283f4e4d096effb,0x5e8fed71a3cbc63a,0x38115f79d8d7f9f6,0x4f5f5e1769c41826,0xf7fc78f35057436e,
0x4abb7d99f14be575,0xdfa81740a10bde71,0x3bfea29df2f11541,0x6f82b1c15d7f48a3,0xe0ffcee0467397e1,
0x71e08acbf53ef49e,0xe84fe8560305c342,0xfa292308f629da03,0x9ab4705e973d7af2,0x891eea7facfaa4df,
0x0b8d7c1c76e8e0ab,0xf067bdda0e9a8301,0xda44df04d591ffb2,0x479c89be7bb96d19,0xf79cf6bfc7c066fe,
0x0dae707af74704ba,0x7f3b5950bd795bce,0xac7d1aee7ac68443,0xecfe6ea53ee7a6a1,0x571e7759be3f7994,
0x4d1aee3e8d4fd93f,0xc051c13ef154ef14,0xf7ed68e1170f6ab0,0x55d205ea7ff75f1d,0x6e1e209ba026042d,
0x1e09445f3d86082b,0xdbca10ff4a7b43c2,0x04bf8eb35b9ceca3,0xbabf153ffc0b2eba,0x0e653cd408cec7fd,
0xa339c8c10638ef5b,0xffcade34bad1e60c,0xbbbf19ca79f8f388,0x2d553fd57cfacd7e,0x7c3a84476da10753,
0x69b204d7591cc2f6,0xb41e3f04d1aa4d4a,0xeac701597faf78d7,0xab47e5d775ce39eb,0xd9d5412ee0bf2b78,
0x3346fcfc1f29df81,0x1fcbf84a15f37cfa,0xdbb9f842c2e99ad8,0x729f39f8cdcced02,0xdc560ca3f59c5b9c,
0x71aef651e358b58e,0x02befe35dceb1683,0xe7a1074ad4553b9d,0x58fc2ee48d09e5af,0x465716d81b5f4ed1,
0xd2713b83addbecc2,0x0d8567650d85f9fb,0x8462847cc06b28bc,0xa09459cd726b6894,0x4ddf5be60c08fa3e,
0x9f8b18465d584dfc,0x794ee04ef3beb341,0x6ce6fb4971e6914b,0xfd1e0b8b108ce773,0xd5bbd8a1281bf08e,
0x9de76bba64f9b205,0xf3f7ba8d159bac7a,0xfb684dfc2399dac1,0x58dad76eafe9b9cb,0xde77e2bfe7e203e7,
0x1fe6ab6895f8ced4,0xe05efeac53dce675,0x7f4d183352b48d63,0x99ded2f995c9fdfe,0xb115855c758ac7f3,
0x8b5f0fb9f98d6cfb,0x2f5060823f80a13b,0xfcd79eeabb267508,0xfc6b57a56f34b7f1,0xe6fe0c11f457c69b,
0xc11552519cfec2e0,0xa8f9a95af8f62beb,0x397f0efd763a80d1,0xf39b3bff4702c43a,0xc073fb3f9afb45f0,
0x4efcf6ffdf768d6f,0x93349b25e5e7de1a,0xa79789a4fbec0c15,0x35b5fced4fc1b15d,0xf8ac18f9b52c2f83,
0x2fd27728f07c6977,0x01be715fce46c52b,0x01d8eec9cbdb7bce,0x3ad94d4d27657cea,0x5fbe12e3cf754f1e,
0x709c8c7e1a13b9b2,0x6abc76f5551e7756,0x5f34d07f8f619468,0x55383c6bd4e0c571,0x94df1de7af79a251,
0x44d6175ccfc292b1,0x2885bbabd1987a8f,0x9fd3f1a9b60e9aff,0x349fd38a3f87bf15,0x9a5f5691fb947719,
0xbefc7359606c6af8,0x78e9be3967703040,0x9c513457be16605e,0x93b9542a34de1f4e,0x68c70526fb48c7e7,
0xefa0846d753b296e,0x6f2315fccd65dc50,0x9c2096e77f642d07,0x4085ede33bf80f47,0x1a3cae304a18f3be,
0x5158d159d44d4d7e,0x66b1445704a7f2e7,0xf08f8e7bf6511584,0xbf6f9de059dfdaaa,0x1efa3fd2ddfb7cd1,
0xabfa77b474f779c2,0x757a1b3aa8d4cd7c,0xe368f8a0f3e6ffed,0x69f50737cef63ef8,0xd0ef34635fd41966,
0x9ef2fcef9a38fa53,0xd99e5af737c3bf4e,0xfd74df06cc541346,0x05380cbbdbebb9da,0xfbaf2bb29c54b3f8,
0x368b54e462895afc,0xe0cfd5435ec0e0e9,0x4ed0f78366665d95,0xeb1f4e13829ce96c,0xf2697df8d7dadecd,
0xfea7bf8ce205b473,0xaa60a87d691f9673,0x78cac5c7ea297ffd,0xfa0daea503066bfc,0xf5de28e76f1b5d2f,
0x504a054a1efe168f,0xc182037f8d6329d4,0x18a4277502f44e0c,0x7ef84abe04eebfce,0xa36ef97920d7ebd6,
0xc2b9dff9f50782e3,0x56877b43a80ed7b5,0xcb77b013bf0dc82f,0x472cea9774fe395d,0xdc3b5e87dab8fcce,
0xfe04d46bd61de6b0,0x1381149e3e772efc,0x56e3d77495ec25ea,0x778e3bd75ff34319,0xacbdda342454d22b,
0x37f9fbaabf1df80f,0x4fc73b48c52e0a40,0xbfc5a11fafef44fb,0x8d27ea3e6ba3ba2b,0x195c60c79c208d0b,
0x8a932f7eaa7d732f,0x70fdf0bfb155d59a,0x4729d410578d35ea,0xcb35f8ca16af067b,0x1067a1d0aaa7ce5a,
0xeb7f9be6f8e555bd,0x0b728ffc512cce1b,0x7737a9ada3e0be13,0xaa307960bef9c23f,0xb27782f9edf496ef,
0xa1b766904d27a2eb,0x1fcfceb6f7571455,0x5eb2ee0e76358e34,0xdfcfba7bf78ae6fd,0x3979428c345a736f,
0x7d7f673794235eaa,0x0b1b79fd6f9dcd63,0xbfa0fd6e0c0609ec,0x3c2135b44e681418,0x17d9e7c7557cff50,
0x41826acb72e4de33,0x286359b09bfebe20,0xa4d4df05e738fe71,0x65d59ff3b9acbb33,0x7e97b8ebf478a2dd,
0x38ac25ca7f605c04,0xded01f7f4dccef39,0x17ce44d469bcea2f,0x597fca26ce3ebe11,0x4469bc1f6f74bff5,
0x83e72f0faee6ffe6,0xbe7e10b530b28ede,0x6e51ceac56284104,0xdd1c146ca7e71ce0,0xa5f6972d1e3411f3,
0x352ee8cf677cafc4,0xd2053fd907ca10b8,0xf3ef505f3ffbe3cb,0x9b8e335d6d2ef674,0x89dfdbeb8e715878,
0xff0d1104eedf6d33,0x5976693bf9f64b5f,0xc4de68f8ee3bdba4,0x7ff9c26b6b9bc566,0x91ac42461e69a6f9,
0xf4b57e6eb567137f,0x37b4232cfa7d5419,0xaacde3e97f7ee01f,0x817b98f62b59dac4,0xe0aeb7c683dc7719,
0x40b1855c3a94443c,0x1fb5fca7f04783ed,0x7d1e13a8259c56e6,0x73cdea0a58c787dc,0x8212e0b822be0d96,
0x6af5b942ae0bca78,0x28f8f0b39f5b1a95,0xc65ce615659e2bfd,0x19daf7f5f374794f,0xc10bfcc70969f786,
0xfad4765ff6ed7a83,0x36510541747e6f37,0xe833bda0e3f69577,0x8fe1ab753a329539,0x4e0acb0da478f374,
0x213ddd1c085fc1d3,0xb283a9d8d4107437,0x7d1a2a8d2b72b7d4,0xa7ad7b87e033847f,0x64f59421551a0ebf,
0xdd63aa84794aff14,0xb811f4d4b77f1ce7,0x7e8cc54afddcdf01,0x9ab79dcb5a9f4ae1,0x08d9d7ec83a0fd28,
0xab358a0ed7875388,0x21e7830e8c76caec,0x95ea35bdaa21d1cd,0x7cfcebde8f062859,0x7b5a6fd079ba05bb,
0xaf06c34ecd4eeb0e,0x8cab28304182776b,0x65efcbc4b033fd63,0xea7acd0b6ce1cdff,0xf823d9fabe6fd67e,
0x97ebb967d3580f39,0xbff87b47822b8ef9,0xc67fbe1e162d359f,0x53a8973e094ea03c,0x95f1b2c3c217b479,
0xf4a71c77edafd539,0xfbf47d6c3c34337c,0xdfff01acab9fcee3,0x43a3d29addf2c306,0x6f389a27015ebe71,
0xb7885cdfed157079,0xa71ed09ebee9674b,0x23cba35742c6297e,0x05be97aa737ca697,0x7ea343e8bcf77fe7,
0xe360bf8271508ea5,0x3ab1c7e2b7acf2ec,0xbed2f1d37bf3b2ba,0xad95f29e8d029ac7,0x357cfefefe15535b,
0x22fbdae77f194102,0x05ea5dcd66aa38f3,0xbb806abfcdf1c1f1,0xafbe73b978469f58,0xa9f2ee7225d94e3e,
0x39f7397ebecf6823,0xf9ba756a89f37cb9,0x707c787a9ad6d1f3,0xd99ebad65d34d2f8,0xf8cdf9af7156d4fe,
0x8659fcabb9a30ded,0x7ae7fac3beca7d7f,0x0dba842e0bd1c08d,0x2b7ca755379c5adf,0xce5d63da2af43de6,
0xf9ba6b34bfc63e82,0x6ad5359fabf3fd5f,0x29f47f3decd650f5,0x501ada460e8f1f46,0x53aa9dc1e1ed6afd,
0xa13f561fbdbd7cfb,0xacf77e09fbb6341f,0x3eb39dfed68a75d1,0xc7684ab7c705afa8,0x337f44dd54b1151e,
0x0da443c950719e67,0xa1fa59cbfbc2ec7e,0x3c57fc1b261ee961,0x1debecc148e35e96,0xd07c073eb6911ff3,
0x65a1373847ceca0f,0x9c67df1ac62a9f75,0xbf8ffd9ea5e7ff79,0xa4fbf370f15fe6a0,0xbd1fa8cdc097312e,
0x86777e3e4eda34f0,0x42ce7d197b8ec709,0x1edce70a71941158,0x7bf085ebfc2ad97a,0x8eb01a9c2f7af450,
0x43da9ea61d234e5f,0x4f6ad131fdbbe6f9,0x5ace0c3abe11b77f,0x04df69b98ad8e10f,0xafa37c19ba7147ad,
0xaef556269023c08f,0x6b9bc56b38e95c8a,0x80cd19672bb82bc5,0x1ede511fe71eae1a,0x3f1a765392e1567d,
0xf3b84733eabc1288,0xdbb98aff6827105f,0x979751fadbfd39ed,0xb28c68d4f533fd51,0x4aa68361ed3dc51f,
0x3fadfa293bdaf876,0x5752cfa1d4ba71a7,0x653b43baf5159bc0,0x8105235eb350454f,0x5fd41b0fa6f3ee7c,
0x26bef3bdae779cbb,0x811ecebb1f9dc3bf,0x4673b844e774518f,0x07dbc63cb57f7f6b,0x1587defdb93023ac,
0x670634df3e907820,0x58af0ea3b4afefdf,0xff3fd6cb1fcfac71,0x93827d2c546bbbb5,0x4bc1f3975e53e9c2,
0x0c3bfada2bfcdfcf,0xafb2ff4b3bdf537e,0x8e153636ba175347,0x3795d4d6b68f04f6,0x0b7d1cb9bb4406aa,
0x5dc210bd9ee05ab2,0xd5de03e3412d6513,0x508f8ca04d19a878,0x9789d9f05609be5b,0xbaaa36f0a88afc21,
0x1cd27144d67ed018,0x5cdfa08a36f08328,0xf0bff7f1ad935647,0xce6f15d8d58ecc5a,0xf89c409c97fe5dc2,
0x0fc11f85dd4effad,0xb6df388f658a0a7b,0xaef6896e0cf67eb1,0xa6b3dbe96ede02b1,0x467344b4abbcb7d2,
0x09bff7fc7ef94e6a,0x557183f5dae7f3fd,0x2f7814e096737f6f,0xfce45b3979c7fb04,0x5ad8fa3d87e5ea9d,
0xa3e2b8e779c4b7dd,0x636b83e328234df3,0x5e09a5b5be9ecfc3,0xae9fb479bedeffbf,0x1cb35ea0cfc9d41f,
0x6a887ef49425cc51,0x8d28547580f95c23,0x817eea84ffab79d9,0xbd6bd478165c7e6f,0x9de3af1a297b5440,
0x7ff9f59c66fe7bfd,0x68a5944708ca245e,0x6def5d0fd043dac8,0x4b7797335fd93832,0xe593831e7e2b02cf,
0x09fe9062b9efd5b9,0x2bb3ff3bc164246d,0x70239f75ebf85edd,0x1f3795ee422551b7,0x063452f688f3460e,
0x85cb3fccd67f46c3,0xa3961817d958417c,0xaeacf4fa1d543c7f,0x97b59bddb28efb54,0xf20a3383284a67dc,
0xaa9d7cf7b8f35d7c,0x609789d4f7d1f467,0xe756a0c79736adff,0xc09416d1ca3ab87c,0xcbf52ee1b915f1f9,
0xfd71018d04d2f7f7,0x99cbf82f9618259c,0x76b87956d66f9f59,0xdfb8fc61d128e297,0xce3c5223a8f6bf9a,
0xb5f144bc67682341,0x199ef0bbbff020c3,0xf49fd7715f944784,0xf7473e9777f38a9c,0xdd34deb6858fe7f7,
0x8cecfb27570235b8,0xcde9ab2da0ff29f3,0x15e608fe5cf1e7dd,0x020bcc2fe1fbe51a,0xea6ce5da5f5d4d15,
0x71d93eb691571f91,0x1afbfd74ee1fc3bc,0x737c3f57a3c51d1c,0x75ca577d6e5efed2,0x79eefc060f79fd34,
0x84234d19767dc949,0xcef1c7817dd65cfd,0x1f4fc452e34fefef,0x37cf6fbac4effcb9,0xede12edeeb2e827a,
0xde53a9cd34d5e8e5,0x723013515a4faa9f,0x7e7145f0741b5a7e,0x75d01974d08478fa,0x38559c8827bdd1da,
0xaa51d47680afbdbf,0x83153f3fb3b7d9ee,0x68fe823f2e9a17b4,0x6734547476943ca6,0xb210455396826f38,
0x276b5d53e811f29a,0x348ae5d7ee3f065d,0xc571c1f1f5d8b836,0x37f751528d4a7ded,0x9ae9aeb79cd77b5c,
0x137539aa95a597f0,0x5b0412df58e73bad,0x959115fb0fe7ef37,0xb5e9bf3fe0dafaf4,0xb7b791fe095eff5f,
0x57ecebf8f9be9282,0x0cbbc1814d78463f,0x52e8b94d7cd95e65,0x12f68f37477f5ef9,0xbde60b19bd470fbc,
0x2de027db3f159b94,0x10f7c98e34518d1e,0x341f0fa6b1463590,0x7cd91a512a0caa2d,0x1aff1f62f07685d0,
0x4df389bf2962d1c6,0xbdebd55414826abe,0x727fd1f725b605a3,0xcff57a6a77350415,0x1fb41284b98a4e0e,
0xd085c7f66d97b9aa,0x87742ec60dbf5391,0x9fbb1c51f6bf820b,0x92b658304dfcefae,0xf7e3a5fde76a7ad9,
0x8456d2a94d4bc47e,0x94c3adafe6b6f6a4,0xc0be75f29e0e9a41,0x02692f070d28f29c,0x7f084efa1185fbd1,
0x9cf5dcbbab86b51d,0x06c3bec67ed04d1f,0xfe3ac72eefb479ff,0x259847831dc7749a,0xc73aa9c7e6b35974,
0xfad98365d9f7b5bf,0xe7b9780cfc732ed6,0xce359ea339be9f8a,0x99a35f3ef04b3f6c,0xb04fc41ccee5767a,
0xded08304f789bafa,0xb0829be5cf83f472,0xce754e0fb48c13dd,0x577f37cd479411de,0x71cd5b351f5bf04e,
0x2d632a9ed1d04df0,0x2d5397f506696e2b,0x158ed82d94a2c538,0x3067facee5e78e9d,0xc20e5e301ebdd074,
0x704d5a6b08a4e25a,0xce8e3f349dedbc40,0x9cfeed87eab6bc68,0xf1a168e696f38e9f,0x551d8cbc469ab74e,
0xf2cfdaea513a370f,0x9cdcdfe16519d8ee,0x0e95eef60411f5bd,0x8de587e8e083d8e7,0xd74e16efb7cd1b79,
0xd4e8c09bada39425,0x169ef7847bacee08,0xa5c1d62b6055aabe,0x05fd2dca2ee0bd9d,0xd22b8d07ce77fb3d,
0x9f9d47aa9fd5ca48,0x368f9a8f35fe7b9c,0xf2b7d2c4fdcce2d8,0x47cfeb8af288c226,0xf9c5d1e8360747b0,
0x0c08f06c99c25de6,0xffa1dfa50471ad94,0x708830fa770f3e90,0xef3d7d590dd11831,0x473ff33582115154,
0x76c3fbf5b1de53c2,0xe53e94d359bed765,0xfb9a5029a8329e7b,0xa04fa38779f7c7eb,0x6b3014ed06c04bd4,
0xfdfc77a691ed7b78,0xcfefe0f29f797696,0x0b2ffcfacd7d8fad,0xaf6ab055a7a6b0bb,0xce9159a9ef16f1e5,
0x5b44e7753b3b48ca,0xde0f155fa12dec4d,0xfda9fef9cbe4d298,0xfa3844728183057d,0xa2a5dc2e5ddaba77,
0xd63fcb037f58baf2,0xcf604deb6ef8e35b,0xe9ffdd7ba7d5f973,0x1f061d4683f957bc,0xeea2a709708eb77f,
0x8d09dd084efe6914,0xd7d79a8f35083c9c,0xff4836d5fd5dace0,0x977839a96685d8a7,0xed2f073f15158fd2,
0x7758e3588138c6b1,0x9bf4ee0e7c7b5f8a,0x7dc95d9f855daa48,0xebf5de3ac7836ef2,0xee6b38e9c5b67bf9,
0xc83b421ed1df50b8,0x31c64cea365da44f,0xdfcb9a659ce33f21,0x7be71b39e08a3e0e,0x70ab0527d54bdc3e,
0xfee3f836d874756a,0xad811df0633ccd67,0x511a0304bdcd9775,0xbaffe087909aab6c,0x73aeb77a1f6656af,
0xabaf8e7b6a6ba7bf,0x91fa08750518258b,0xdaeb675f80c7fa3f,0x8473e6fd375f137e,0x4a6e8e2a760bbccd,
0x229ee6f75e773ec9,0xa956f79d7ce0cbce,0xa53aeb784f23db1f,0xc16c771bf4e7fa05,0x14f04e72d4e7d1e7,
0x3e8edd56c7239832,0x9d66af4b7102c20f,0x8e3d9ee34d7b418a,0x238ea0d99ed749df,0x472ecca138c507e5,
0x8ea6a08a9c0b3579,0x139d7e7f45362d7d,0xe2a9c4d20dbab7b2,0xc19a35ea05bb2f51,0xd5f847ff3fdfecc7,
0x59f8471ad5be3f6f,0xb2af5b55c752b5fe,0x03344cae129dc99c,0x7f84eff1cd63a31d,0xbc25ff35378337f8,
0x425cf97eeb6bd91e,0xd2d6369d2f847975,0xa8f08af6a9dafb3e,0xdb44facef05f3b9a,0x5d3ff07942fbfd46,
0x9f4b72f3ff063410,0x7b79bf4b0d98deff,0x4683023e823cfada,0x77cdd7df41b451c2,0x2a3cdfc051f9747b,
0xce099df9dd27a7e6,0xcbd44066a6e97968,0xf0f1d3fbb419ffad,0x603e68c2ca38ee29,0xa3cdfec111de8a78,
0x58da12bb9d539f46,0x0ab87e50cde6a105,0x0ab37b8576d2b397,0xebe6a0c3783054d4,0x236b478f2863946b,
0x56c1722c18cca3f5,0x94d6d3a97d4cdfc0,0x7e1b56a50bad99ed,0xb7682a7d73e2a506,0xda6be1085f3425e2,
0x5ac31dbe0e839ab4,0xb28d961bdda6a114,0xb187451ed70fed14,0x88d2ecaeee751831,0xaf7e8e35decd5c88,
0x2a5fdcd455026e96,0xd74a180f4ee3f9f7,0x591fda3c12b7d976,0xb8fed3a827b6a04d,0x5ba0bf3f1d6463f3,
0x7467d90268ce2047,0x1df9cef87dd1ee8a,0xc4bf55152b56ea75,0x3dafb467d20e9770,0x75dacf61c7514efc,
0xc1596d1a5bd81f02,0xdd204bb80dfb7bdb,0xdfb87ce6eb23cb33,0x9ba053487ef53f1f,0x0ef3bb7d92082339,
0x7a882b2c11cd4b9f,0xe7707b26ac9fb43c,0x9a3f7cb9abdee358,0x3693743c1f6b28d0,0x5f7f95e855f8e35b,
0xc07cbb87bfb03c78,0x17f8f813db5f066f,0x3bb2fe89cee9626b,0x6d29233580f4728e,0xf855c67d469bca7e,
0xadb7bdba3f6282de,0x6f7c7f0841b0d06d,0x715fe05aab3de340,0x7021197d032beb6b,0x5bb1f3eba29683a6,
0xf8fda3813471d61a,0x76ff658f0bd8dfc7,0x59745a9e4fc50bd4,0x19ac1556e73f6bea,0xa513c183a5e3e5fa,
0x4060f025822b9edb,0xadf816467efd4562,0xb848ebde35fd4fbe,0x38a5e1671af47604,0x08ffb27efad98309,
0x1e1d6cb020ca1482,0x59edd233ef5aa9f5,0xd3efe342f2843576,0x51e3f869e5b535b7,0x8c1feccfa58bd9a8,
0x15dd3418268e3f7f,0xf5faddc0d8b584a1,0x1db1fcbdffd2ddcb,0x0c2ed158e5c8790b,0x07fcadfa6ebd2724,
0x6cce8e1e256f52cf,0xe5fdc0be7830f787,0xeffe029bd7cfaf9f,0x48e2b299c053fdfe,0xdb39a859c770d2ef,
0x6f7f60b5ef2b78fe,0xdbaef01ca75b37a7,0xe5288ec7cfd59671,0x5f02508ae3aff93f,0x61586b504bf7473e,
0xff62d6de095ae9c7,0xcf5daf95604e42ca,0x9bdd8c2cd1cdf1cb,0x395bfa8eb2f79f8f,0x3ebecf19fbf3bf6c,
0xdfd94c686eae866f,0xce05ef8ac6be44d1,0x3c9b8a135f6911c6,0xbd4bb8d390b10519,0xaaebe09dad144737,
0x383fce454a6be203,0x15282d38ceaf4a3e,0xc3a17f6d9fda33b2,0x799adb5cb42f6bf9,0xace48f06383dca8f,
0x7af758ced6e0b8e4,0xf2d6efb72768e9bc,0xfdc08a23467cdd7d,0xa5d93f6d7053549a,0xeb71e0bcadadcd57,
0x47ff9f55496fabf6,0x4f877ea6aeba0c17,0x025d5da9b5f9f8e7,0x982587f9f58ca3ab,0xef85c082faf1e0e6,
0xb6f07c3dcd412ce5,0x9fdb6fd5f633fb77,0xf01f9fbaf7504d20,0x4b7becacade04759,0xdf4de7de1a2ce5d6,
0xf70f3fef0e902edf,0xb5f7556c1809f9e9,0x5b655bd3f6db6a68,0xfdb9b8ee5de3ff99,0xe8ef8d614a56fecb,
0x72e611ed509bf3e1,0xf7f7fe6e4e7b7e13,0xabe35b8454dd3580,0xfc108c77a3cdfc14,0x6557eeb99fd153bf,
0x4aa11a9cde0cbeef,0x3faf608ac5696a6f,0x6b2d4a6a0fd6fe1b,0x3962fe13a869e71d,0x0b5f75fa04fd9b78,
0x2339be39f64d2c83,0x7fb208d0bcfd55ea,0x59ade3f86f6b09da,0x9533bc181623b208,0xcdf04d66e9678d35,
0xcf80e5c9bff629f7,0x97d2b0f7eacbe204,0xf7bc1b4ef1422b0d,0x5d3262b94137453a,0xcd7cd52c737ca083,
0x3f6828ca153dfd94,0xd639a5ef7af46935,0x29c6972f3ddadc70,0x20adbd5f1ed797f5,0x1e3f8ca3364df0db,
0xeaa09f4bd23f9afd,0x74f37e49e76b8fe7,0xdd1a7ed0bebd66bd,0x2edabe5e31ff8302,0x6ce6f15f1fcf4271,
0x75f60d5dce1066f3,0xdde04e2febd4ef1a,0x041d7c52c4a23e3d,0xb77b5ff5683cfeca,0x337daa3041e3ad8f,
0xb2916b131cdce7d7,0xb82fca79f75d35f5,0x2fdbe1ad4bbc7074,0x556302326343a839,0x3b6ee6c9c8585eff,
0x07da296e725dd275,0x7a3ff28f72fb0f95,0x4ca3cb5947bee8c7,0xecbde50b783ff8e2,0x8f64fac1b030732a,
0x9e1df685913ef026,0x92df5e7b165079d4,0x4b1516e8f1539655,0x2dacff9182e19c18,0xd0d08d385dce8cc0,
0x7e7b9e11e176feb9,0x836bebb9fe6f2eb5,0x3eba38684d7c11ed,0xb7d4df9380a77158,0x3605d74e77c3df2b,
0x2a3aed772de9d4a3,0xfeae1463f9f8c72b,0x506c0c7d6684d27e,0xd357465bf23ff64b,0x0f088d4adcbaa69d,
0xdb0915158d3bc57f,0x9733fce460b75fe4,0x741065ea5edd7472,0x9cfbcef9e735ebdc,0x56f8446a6a70563d,
0xf15825eadc177e7d,0x6771c232ef2f6bd4,0xb3b780fe9da7e3d2,0xcecaa27765fed047,0xb4fde7162bd41073,
0x39a10b4ee50e4ff8,0x26b3b87bf1479c40,0x943fca76e83f9fe8,0xd4ba17f4fbec05d5,0x0c7b748ce7b9ee45,
0x2b57e7104e36a8f6,0xf5b0ae3410f73ef1,0xc752b742f38d87d1,0xcb7135fad541b447,0xce083a7b291f06de,
0xe35f102cddfc2e8f,0xa5fa05747bac34f2,0xe4571fdbaceaf456,0x7dccb58409ddab3f,0x1159a38cfceae2a9,
0xaf825faa38e04efe,0x51dfe107051315f3,0x739576833eb3fbc6,0x7d20346a28f8efe8,0x0e1706cd5fbdba5e,
0xe70f135b0ea5beaa,0x1ea42fdb534b7471,0xe735f5855877f282,0x751702fc7e096759,0xdca345e77383358f,
0xfdaed7ed70234dea,0xfcb0b4ade0f7c7b3,0xf59ace3ed2271154,0x667e2dd1d65bfbb5,0xea09f78368cd9cb6,
0xaf47ed5b059306cc,0x4615083d7bed50f0,0x659e5028ae1211cd,0x66b383084fa3fd6c,0x6b1dd2e84694a9d9,
0xef2a94728cc3a3b7,0xbb733f7f3ba5ee73,0xb2b1639fd9dd4d7f,0x3eabaec7de0cd525,0x3eb768fc561a07db,
0xc13da8739b9c076f,0x17a0d9b49dce26fc,0xf38f29c72f79bd93,0x7cee9631f5eb6b4e,0xc99a370f9eb2d06d,
0x76f1fb7c51142559,0x98a5c7fe6e95c40b,0x79adad2f2104b9bf,0xe7bdfcd187d3cdf6,0x05c0f3641d2b5284,
0xa714469eccdc77ef,0x8d7aa36f3fd5d6c7,0xa241cd5b1adc5535,0x06776c17595c5bae,0xdbac4ee5dfc72854,
0xfd92bd43ab408a3c,0x7f35e8c07cf7a85f,0xd7adaa44effbfe6b,0x06cb75ce6b661f01,0xc565e68469f84fe9,
0xe058a9bac7371ec3,0xaeffbe50418ebdb1,0x58eb06ccce17d3bb,0x2fd710eea65d9b1c,0xf67e1dfcfdf42a33,
0x51e73be0977d7f47,0x1a179ff19d9ada16,0x39529bfcde697c79,0xbf3d6f947eae1f97,0x3353c13fd454c69d,
0xc1d144a09c7cfded,0x70605f563e84d3c1,0x8a39a9dcb6906c98,0x21579c714abfcd46,0x8775ffb38f3915b3,
0x0f1e7efec91d8d74,0x74729c7748d07956,0x551c3d9eb75023d9,0xd0536715f8a75684,0x66e6a775e6fe5f42,
0xcbdfcdfe769f7854,0xaa71d59743cb0ae6,0x53f0d0dae3f5e5aa,0xfd1c0be3d2c7f50f,0x241472f15e6f87eb,
0xcee382badfe67ab6,0xa50bcaeb595be107,0xca228e6fed7c70eb,0x6a8d98d7fdd96cf4,0x705e66a095fd8422,
0xb3bf974ccbf670bb,0x468f05b982fdaa31,0x5d9bdd9d85109683,0xed70677fa7de3ea6,0x53e95ba0f80d419c,
0x934aa6c973e72f66,0x115ff02de0cfcaa8,0xe1e441edfe752fec,0xc6e8450737a4abb8,0x30dfa53c9c567efa,
0x283ee2fd6d79a2b5,0x6ea0c7e938da13bd,0x8137e813abd0f560,0x387fb994794fe289,0x5af7a71ccb5ff535,
0xb789dd413847b0fc,0xd4105418d6aced63,0x3916e70e97f483e1,0xdf6348ca3bfa1d95,0xff94f2b6571fe6b2,
0xed38e8488ff9c6db,0x7f4d7bf38107b940,0x5bc083ac18ed3586,0x9cf4a5a771df1f9e,0xddedb7b1ae26f82b,
0x6a979aff3d9c504c,0x7efb4758314b4d7f,0xb1c8c71dd5469607,0x9ac2fecf561d1817,0xf58a9da5ad9c066f,
0xb2e34b70b2209779,0xc5a5de0cad560b61,0x68542509ff764184,0xbb6d77ffd079c6df,0x5ace69e51bc952fa,
0x2bb3c7ff35ea59cd,0xffc52e7baa07dd25,0x03f38b29d9d68cf3,0x94d7afd75e8d9df2,0xff75adbc423a0f8a,
0x6ceb04128969dd61,0x751ca7fdf9fab9bf,0x2b09f73b387c097a,0xabece7fdf3978af5,0x8ed1ed138dae0b79,
0x79f650655027fcdf,0xa5bdadcbf2b64762,0x8d62e56feed6c183,0x60fefe08e6f3abfc,0x0575c27b1d51cded,
0x4735fca7f0ecbc7c,0xb91ec3811c1b1dcd,0x4ed4d8f47d7c418d,0x1cb3ac706ce41855,0x823d87b4f2d34ba1,
0x9bdd23ff6ab747a7,0xf55421fae8365c3f,0xbfcd5b3472fdd199,0xbe1571a35ee8ea6f,0x80f4b7fb23826904,
0x081375b597d6f689,0xab6df7dceb7cf40b,0xd19dcd20b4ade6a6,0xffe29bbfb3f9e25b,0xbb1cf616c3f5b775,
0xfb72ed1e53cd456d,0x1cdce72ac509efa1,0x3501c3f7f7a946b0,0x7db0567083aefd2c,0x5186ce2b8fe5fbcd,
0x4785aefd23cb508e,0x1b9c99ddf9a87f18,0xac3dfc758418af38,0x5a99be7ed7a427eb,0xeccd0fa715ca71ad,
0x4e8cd195bf4f45e6,0xa3cbee78f0566be1,0x79be2973ded74258,0x2dfe9c7bcc706cd7,0x34aece6a6f9d41f8,
0x251b983ca6ad6f70,0x5ff3f6575aabcd8e,0x122f8fe7d51e7a50,0x9ff18e70b6bd4fba,0xde7befbb42ffce11,
0xafbf02c8d8eea3d8,0x18e50fd6e59cbdf8,0x423aa4dd0159aa09,0xf7b2178784708c56,0xd07b57f7f5ef15ac,
0xe2a572fd4a7847e8,0xcf8a4ee70db74e0f,0xf832bd6a5168e3da,0xff3514b9da6f6c13,0x92ee541fec9ad806,
0x0f96ee7759bf8fe5,0x8d1a2fe03edb24de,0x71015d50c7fecc3a,0x595279c51979e304,0xde6afcfddc2b3506,
0x03460ebe3939cbcd,0xd04d9062a6359094,0xa9f2bb4fc74fbf8e,0xe2818a0982b2b5e6,0xa1ba1e7eff2a66fd,
0x35022ab179f9d47e,0xba739aa72b7db737,0xe1d19474bbd42041,0x79ce0e3c7727bd15,0x83cfdfdb654a05ef,
0x07827094d05aa6f4,0xb22e0ab8d379ef36,0x71f37031ce7eeb79,0xa84758ebbe8609cb,0xa8eeb1acded6f042,
0x2a6a51ff56f97bf7,0xfafb02690ab1af7b,0xe7bb37cfafb1daf8,0x9767a9f3fbc66ebb,0x39aada3e17587b53,
0xfe3e8a74bbda0f1e,0x7b0c34fcbfeb9dae,0x97409ff2e6929a5b,0xff3a8ba33b5f3c9d,0xd0abd772387bf147,
0xd9bd38ed2ddcba2d,0x3ddc3ad93fadbaf0,0xbb41cf4bb531f9cb,0xc13467de51ee8a3f,0xd2d57782d054e751,
0xaef7379e874d75dd,0xcd7b1be772cf027d,0xfecdf7a8d1c1b33c,0xdf2beb1f41066f9c,0x8b95f78dbde767e7,
0x1ea6f8167be66e9e,0xe75bdafe12268e09,0xdc7eae1a5fd9cde1,0xd5be7ed1c7b1783e,0x5dcfaa296fa4d759,
0x1e35b41748f1d907,0x8af4b2b641d6f6fd,0xe581cd27f572e1f6,0xb3e4ec29259a8746,0x0e86cfca7816fa34,
0x2af4ee2978adb2b7,0xf683e5e4f1b558d3,0x5d01b74b1787b,
];

#[derive(Copy, Clone)]
pub struct PrimeIter {
    pub ind: u64,
    pub end: u64,
}

impl Iterator for PrimeIter {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<u64> {
        if self.ind <= 2 {
            self.ind = 3;
            return Some(2);
        }
        if self.ind % 2 == 0 {
            self.ind += 1;
        };
        loop {
            if self.ind >= self.end {return None};

            let old = self.ind;
            if self.ind <= 255 {
                match self.ind as u8 {
                    3 | 5 | 11 | 17 | 29 | 41 | 59 |
                    71 | 101 | 107 | 137 | 149 | 179 |
                    191 | 197 | 227 | 239 => {
                        self.ind += 2;
                        return Some(self.ind - 2);
                    }
                    7 | 13 | 19 | 37 | 43 | 67 | 79 |
                    97 | 103 | 109 | 127 | 131 | 163 |
                    193 | 223 | 229 => {
                        self.ind += 4;
                        return Some(self.ind - 4);
                    }
                    23 | 31 | 47 | 53 | 61 | 73 | 83 |
                    151 | 157 | 167 | 173 | 233 | 251 => {
                        self.ind += 6;
                        return Some(self.ind - 6);
                    }
                    89 => {
                        self.ind += 8;
                        return Some(self.ind - 8);
                    }
                    139 | 181 | 241 => {
                        self.ind += 10;
                        return Some(self.ind - 10);
                    }
                    199 | 211 => {
                        self.ind += 12;
                        return Some(self.ind - 12);
                    }
                    113 => {
                        self.ind += 14;
                        return Some(self.ind - 14);
                    }
                    _ => {}
                }
            }
            if self.ind % 3 == 0 {
                self.ind += 2;
                continue;
            };
            if self.ind % 5 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 7 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 11 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 13 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 17 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 19 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 23 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 29 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 31 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 37 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 41 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 43 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 47 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 53 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 59 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 61 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 67 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 71 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 73 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 79 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 83 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 89 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 97 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 101 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 103 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 107 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 109 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 113 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 127 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 131 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 137 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 139 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 149 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 151 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 157 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 163 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 167 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 173 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 179 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 181 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 191 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 193 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 197 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 199 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 211 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 223 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 227 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 229 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 233 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 239 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 241 == 0 {
                self.ind += 2;
                continue;
            }
            if self.ind % 251 == 0 {
                self.ind += 2;
                continue;
            }
            let mut x: u64 = 257;
            for change in (HuffmanIter {
                iter: BitsIter {ind: 0, data: GAP, bit: 0},
                change: 0,
                code: 0,
                bits: 0,
            }) {
                // print!("{},", change);
                if self.ind % x == 0 {
                    self.ind += change as u64;
                    if self.ind - change as u64 == x {return Some(x)}
                    break;
                }
                x += change as u64;
            }

            if old == self.ind {break}
        }
        self.ind += 2;
        if self.ind >= self.end {return None};
        Some(self.ind - 2)
    }
}

pub struct BitsIter {
    pub ind: usize,
    pub data: &'static [u64],
    pub bit: u8,
}

impl Iterator for BitsIter {
    type Item = bool;
   
    #[inline] 
    fn next(&mut self) -> Option<bool> {
        if self.ind >= self.data.len() {return None}
        self.bit += 1;
        if self.bit > 64 {
            self.ind += 1;
            self.bit = 1;
            if self.ind >= self.data.len() {return None}
        }
        Some((self.data[self.ind] >> (64 - self.bit)) & 1 == 1)
    }
}

pub struct HuffmanIter {
    pub iter: BitsIter,
    pub change: u8,
    pub code: u16,
    pub bits: u8,
}

impl Iterator for HuffmanIter {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
loop {
        let b = if let Some(b) = self.iter.next() {b} else {return None};
        self.bits += 1;
        if self.bits > 16 {break}
        if b {self.code |= 1 << (16 - self.bits)}
        match (self.code, self.bits) {
            (0b111__0_0000_0000_0000, 3) => self.change = 3,
            (0b100__0_0000_0000_0000, 3) => self.change = 1,
            (0b011__0_0000_0000_0000, 3) => self.change = 2,
            (0b001__0_0000_0000_0000, 3) => self.change = 6,
            (0b000__0_0000_0000_0000, 3) => self.change = 5,
            (0b1100___0000_0000_0000, 4) => self.change = 4,
            (0b1010___0000_0000_0000, 4) => self.change = 9,
            (0b0100___0000_0000_0000, 4) => self.change = 7,
            (0b11010___000_0000_0000, 5) => self.change = 8,
            (0b10110___000_0000_0000, 5) => self.change = 12,
            (0b01011___000_0000_0000, 5) => self.change = 10,
            (0b110111___00_0000_0000, 6) => self.change = 11,
            (0b101111___00_0000_0000, 6) => self.change = 15,
            (0b010101___00_0000_0000, 6) => self.change = 14,
            (0b1101101___0_0000_0000, 7) => self.change = 13,
            (0b0101001___0_0000_0000, 7) => self.change = 18,
            (0b0101000___0_0000_0000, 7) => self.change = 17,
            (0b11011000____0000_0000, 8) => self.change = 16,
            (0b10111011____0000_0000, 8) => self.change = 20,
            (0b10111010____0000_0000, 8) => self.change = 21,
            (0b10111000____0000_0000, 8) => self.change = 19,
            (0b1101100110____00_0000, 10) => self.change = 22,
            (0b1101100101____00_0000, 10) => self.change = 24,
            (0b1101100100____00_0000, 10) => self.change = 27,
            (0b1011100111____00_0000, 10) => self.change = 23,
            (0b1011100100____00_0000, 10) => self.change = 30,
            (0b11011001111____0_0000, 11) => self.change = 25,
            (0b10111001011____0_0000, 11) => self.change = 26,
            (0b10111001101____0_0000, 11) => self.change = 29,
            (0b110110011101_____0000, 12) => self.change = 28,
            (0b110110011100_____0000, 12) => self.change = 33,
            (0b101110011000_____0000, 12) => self.change = 35,
            (0b101110010101_____0000, 12) => self.change = 31,
            (0b1011100110010_____000, 13) => self.change = 32,
            (0b1011100101000_____000, 13) => self.change = 34,
            (0b10111001010011_____00, 14) => self.change = 36,
            (0b10111001100110_____00, 14) => self.change = 39,
            (0b10111001010010_____00, 14) => self.change = 38,
            (0b101110011001110_____0, 15) => self.change = 43,
            (0b101110011001111_____0, 15) => self.change = 41,
            _ => {}
        }
        if self.change != 0 {
            self.code = 0;
            self.bits = 0;
            let val = self.change;
            self.change = 0;
            return Some(val * 2);
        }
}
        None
    }
}


