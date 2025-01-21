//! M3 data.
//!
//! The Prime Symmetric Goldbach (PSG) conjecture states that every non-zero multiple of 6 is
//! in the middle of a symmetric 1-or-prime gap, for for a multiple for 6 `n`, there exists a `p`
//! such that:
//!
//! `n = a + p = b - p` where `a, b` are primes and `p` is a 1-or-prime.
//!
//! The M3 sequence consists of 1-or-primes except 2 and 3 that are ordered by extensions of
//! multiples of 6. For example, 6, 12 and 18 are extended with 1 to get 5,7 and 11,13 etc.
//! The next multiple of 6, 24, must be extended with 5 to get 19 and 29.
//!
//! The data in the M3 sequence can be cut off to produce a cover for the PSG conjecture up to
//! some finite limit.
//!
//! For more information, see
//! [PSG paper](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/prime-symmetric-goldbach.pdf).

/// List of M3 sequence.
pub static DATA: &[u64] = &[
1, 5, 7, 13, 17, 11, 19, 43, 41, 59, 23, 29, 67, 31, 53, 47, 71, 101, 89, 113, 37, 73, 109, 139,
179, 83, 127, 281, 181, 107, 241, 163, 229, 61, 349, 79, 137, 233, 337, 131, 151, 223, 103, 173,
149, 167, 367, 97, 311, 269, 191, 359, 211, 317, 239, 157, 199, 419, 197, 193, 331, 347, 293, 439,
397, 307, 251, 277, 401, 487, 389, 227, 431, 313, 257, 409, 643, 421, 383, 691, 461, 263, 739, 463,
283, 443, 467, 457, 541, 379, 563, 569, 601, 587, 353, 499, 641, 449, 503, 479, 571, 647, 547, 881,
1097, 523, 577, 797, 521, 593, 433, 271, 809, 631, 1033, 619, 787, 491, 769, 599, 557, 373, 659,
1039, 743, 613, 977, 911, 839, 683, 773, 733, 617, 607, 863, 751, 2129, 709, 1237, 877, 727, 1031,
677, 971, 1051, 859, 1069, 673, 967, 509, 719, 857, 1709, 821, 941, 1499, 887, 1303, 757, 1021,
1409, 929, 1061, 919, 827, 907, 1249, 1129, 1229, 811, 1223, 853, 1291, 1151, 937, 1091, 1087, 953,
653, 661, 1429, 1597, 1117, 1093, 1019, 1063, 829, 1481, 1171, 1297, 1231, 2137, 997, 1327, 1013,
1123, 761, 1213, 1447, 1609, 1009, 1301, 883, 1321, 1699, 1483, 701, 1193, 1493, 823, 1571, 1103,
1373, 1451, 1279, 1669, 1433, 1721, 1637, 2063, 1901, 1289, 1427, 1201, 1307, 991, 1109, 1759,
1277, 1657, 1511, 1543, 1747, 1049, 1583, 1381, 2027, 1187, 1153, 1567, 1423, 1949, 2273, 1163,
1787, 2437, 1523, 1741, 1361, 1489, 1907, 1579, 947, 1531, 1663, 1913, 2389, 1217, 2239, 1667,
2099, 1987, 1367, 1607, 983, 2131, 1613, 1259, 1811, 1753, 2539, 1999, 1973, 1549, 1553, 2267,
2161, 2029, 1873, 2281, 1979, 1789, 2111, 1723, 1319, 1871, 1931, 2543, 1777, 1627, 2113, 1471,
1559, 1993, 2003, 2383, 2213, 2621, 1439, 1621, 2089, 2243, 1877, 1951, 2251, 1399, 2039, 2143,
2087, 1783, 1601, 2053, 2269, 2207, 2083, 3001, 2399, 2311, 3511, 1823, 1861, 1733, 1283, 2081,
2447, 1847, 2521, 1879, 1459, 2473, 2011, 1801, 1889, 3229, 2551, 2549, 2221, 2179, 1453, 2969,
2237, 2141, 2671, 1697, 2659, 2357, 2411, 2309, 2729, 2293, 2341, 2069, 2803, 2861, 1997, 2833,
2377, 2609, 2467, 2579, 2297, 1487, 2531, 2371, 3023, 2837, 2503, 2393, 2351, 2767, 1693, 2939,
2347, 2423, 2749, 2657, 2719, 2017, 1831, 2663, 3083, 2789, 2617, 3491, 2909, 2797, 2153, 2339,
3271, 2801, 2381, 1867, 2477, 2971, 2683, 2591, 3307, 2203, 2693, 2677, 3181, 3209, 3037, 2689,
2857, 3163, 2557, 2699, 3187, 2459, 3709, 3079, 3191, 2417, 2819, 2713, 3301, 2791, 3373, 2647,
3221, 1181, 3049, 2707, 2753, 3463, 3137, 2887, 3121, 2333, 3119, 3109, 3019, 3571, 3461, 3677,
2593, 2731, 3299, 4153, 3251, 2633, 3499, 2687, 3457, 3863, 2711, 3469, 2953, 4027, 2287, 3821,
3539, 2999, 4007, 3727, 2777, 3391, 3943, 4567, 3061, 3319, 2441, 3877, 4217, 3823, 4451, 4519,
3517, 3203, 3761, 3967, 3253, 3769, 1619, 3637, 3547, 4373, 3671, 4051, 3359, 3331, 3389, 3217,
3361, 3947, 3347, 4231, 3691, 2741, 3467, 3583, 3169, 2879, 4273, 4603, 2927, 3889, 4447, 4337,
3041, 1933, 4517, 3917, 4159, 3089, 5519, 4211, 3607, 3371, 3919, 4349, 2903, 3529, 3851, 3643,
4001, 3259, 3557, 3413, 3581, 3533, 4691, 2843, 3833, 3011, 6067, 4019, 4339, 3673, 3793, 2917,
3449, 3911, 4441, 4643, 5381, 3797, 3541, 3613, 4129, 4243, 4283, 2957, 5011, 4639, 4423, 3433,
5081, 4003, 3701, 3907, 4241, 4327, 3847, 3067, 4013, 4547, 3559, 3881, 4483, 4481, 3719, 4657,
5501, 3767, 2897, 3929, 4261, 4219, 2851, 4093, 3623, 5413, 5333, 4651, 3329, 4463, 3313, 5869,
4021, 5051, 4951, 4919, 3923, 4201, 3697, 3167, 2963, 4397, 4229, 4357, 4073, 3659, 4703, 4139,
5281, 4363, 4931, 4391, 5303, 5209, 4127, 3527, 5261, 4523, 4831, 5237, 4789, 4409, 4099, 4783,
6949, 5791, 4259, 4421, 4591, 5557, 3343, 7481, 5039, 5119, 5641, 5701, 4751, 6469, 4793, 5189,
3931, 4909, 4507, 4937, 4091, 5351, 4877, 5651, 4733, 4079, 5023, 5861, 4457, 3803, 5167, 3779,
4759, 5741, 4549, 3989, 4057, 4493, 5419, 5009, 6299, 5923, 5407, 6361, 5233, 3323, 6073, 6037,
4177, 4933, 4801, 5197, 5867, 4987, 4871, 5309, 3739, 5749, 5099, 4943, 5077, 4817, 5591, 6221,
4271, 4723, 4673, 4721, 4967, 3407, 4787, 4513, 7919, 6451, 5569, 4637, 4649, 4903, 4861, 5059,
4957, 5231, 5101, 6011, 5417, 5659, 5527, 5827, 3257, 3733, 6133, 4973, 4157, 5021, 5347, 4561,
6043, 5881, 4621, 4729, 4969, 5647, 5273, 3853, 5737, 6247, 4297, 5393, 5147, 5623, 5479, 5431,
6131, 4597, 6143, 5839, 5449, 5563, 5801, 6833, 3617, 6199, 5171, 5813, 4799, 3593, 4289, 6959,
7993, 6779, 5653, 4889, 7309, 4663, 6427, 4813, 5531, 7411, 6827, 6397, 4111, 7741, 6263, 5477,
3631, 4049, 5153, 6007, 6481, 5107, 5279, 4999, 7103, 5879, 5581, 5903, 5483, 7121, 5323, 6529,
5441, 4133, 5783, 6053, 7177, 5503, 5927, 5573, 6599, 7039, 5297, 6029, 6089, 4993, 5399, 4679,
5113, 5851, 6841, 5443, 6389, 5003, 6857, 5987, 5693, 6317, 6373, 5779, 6661, 6047, 5471, 5179,
6229, 6869, 5227, 7517, 6203, 5639, 6151, 6907, 6091, 6173, 6977, 6719, 6359, 6733, 6709, 8513,
6691, 7589, 5857, 5669, 6343, 6917, 8243, 6761, 5683, 5953, 6829, 6271, 5711, 6553, 5939, 6329,
5849, 5843, 8831, 7523, 7127, 5087, 6791, 5807, 7639, 8069, 6449, 8353, 8669, 6101, 6521, 6353,
8059, 5981, 7159, 7699, 6163, 7283, 8689, 7703, 6257, 6079, 7187, 6301, 5821, 7433, 7207, 5387,
8539, 8093, 4583, 6277, 6679, 6367, 4253, 6217, 8291, 6619, 6197, 6911, 7219, 7109, 7949, 7547,
7129, 6899, 7237, 7193, 6323, 5507, 7687, 5689, 7507, 7001, 7649, 6571, 7591, 5657, 7069, 7873,
5521, 6269, 8089, 6337, 7019, 5717, 7351, 6983, 7669, 8147, 5743, 7877, 7583, 9029, 8737, 6653,
8693, 7603, 7559, 7541, 6703, 6673, 7307, 9011, 8923, 7499, 8521, 10657, 6287, 6781, 8221, 6737,
7057, 7321, 7853, 6763, 7907, 8011, 7243, 7607, 7369, 7487, 7561, 7573, 7723, 6971, 6379, 7753,
6637, 8839, 8429, 8287, 7079, 8599, 8111, 6793, 6659, 6473, 6491, 6701, 6421, 7823, 6803, 7879,
6863, 7549, 7013, 8627, 5437, 7727, 6967, 6547, 6883, 7717, 6607, 7043, 7643, 7691, 7211, 7331,
9377, 8527, 6823, 7247, 10487, 8009, 6113, 6577, 6563, 9679, 6991, 8053, 9511, 8941, 8039, 8123,
9209, 7789, 9473, 9833, 7027, 10007, 9923, 7933, 6581, 9007, 7459, 7901, 7829, 8171, 8081, 8821,
7489, 8311, 6121, 7937, 8861, 7681, 7417, 8369, 10079, 7759, 10343, 9043, 8837, 8431, 8269, 8363,
7229, 8161, 7841, 7621, 9649, 7951, 8501, 9049, 6871, 8707, 7333, 8263, 8563, 8297, 9001, 9281,
8573, 8887, 7963, 9431, 8219, 7393, 8191, 8741, 8179, 9419, 9551, 8317, 8443, 8209, 8237, 8087,
8753, 6311, 9277, 7151, 10271, 7793, 6689, 9547, 7537, 11789, 9349, 8641, 8581, 8167, 8893, 7927,
9041, 8423, 6551, 6211, 9239, 8461, 7253, 10009, 9697, 6961, 9067, 7529, 7349, 6569, 8761, 10937,
9371, 9391, 9221, 7673, 9413, 8609, 9791, 7817, 8699, 7883, 8999, 8377, 8647, 9283, 6997, 9187,
8779, 10369, 9631, 8537, 9859, 10267, 8663, 7477, 10037, 8467, 8623, 10597, 10847, 9497, 11257,
9181, 9901, 8783, 8329, 5897, 7213, 9767, 10477, 9433, 8389, 10639, 9103, 9157, 9091, 10993, 10091,
8969, 9601, 9059, 8849, 8963, 7297, 9227, 9839, 12101, 9539, 9323, 10867, 10753, 8233, 9137, 9739,
10559, 9787, 10061, 9161, 9629, 9817, 10739, 9749, 8971, 11213, 12653, 8419, 10211, 8677, 9463,
9931, 8681, 9883, 12659, 9803, 9151, 8803, 9661, 10949, 8543, 7577, 6947, 8293, 10067, 9241, 8117,
10301, 9743, 10223, 9619, 10729, 10259, 9203, 9293, 11807, 9199, 11411, 7451, 11519, 8929, 8231,
10289, 11071, 9127, 10069, 10723, 9257, 10531, 10253, 10151, 8387, 9811, 9587, 10399, 9397, 11299,
11933, 9871, 11273, 9013, 12107, 8017, 8731, 9733, 9643
];
