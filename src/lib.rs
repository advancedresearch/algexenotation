//! # Algexenotation
//!
//! Algexenotation is a way to represent multisets
//! as natural numbers with algebraic compression.
//! Inspired by [Tic Xenotation](https://mvupress.net/tic-xenotation/).
//! For more information, see [paper](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/algexenotation.pdf).
//!
//! ### Motivation
//!
//! - Model ordered Cartesian combinatorics
//! - Algebraic compression of Tic Xenotation
//! - Encoding hypergraphs as natural numbers
//!
//! ### Triangle example
//!
//! This example shows that a triangle is `17719'`.
//!
//! ```text
//!                0
//!                o
//!               / \
//!        1+0*1 /   \ 1+2*0
//!             /     \
//!          1 o-------o 2
//!              1+1*2
//! ```
//!
//! There are 3 nodes `0, 1, 2` in the triangle.
//! In Algexenotation, these are hyperprimes.
//!
//! An edge from `a` to `b` is encoded `1+a*b`.
//!
//! The product of the 3 edges is the triangle.
//!
//! ```rust
//! use algexenotation::*;
//!
//! fn main() {
//!     let n = ax!((1+0*1) * (1+1*2) * (1+2*0));
//!     // Prints `17719'`.
//!     println!("{}", n.original());
//! }
//! ```

#![deny(missing_docs)]

use Algexeno::*;
use Op::*;

use std::fmt;

mod primes;

/// Binary operator.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Op {
    /// Addition operator.
    Add,
    /// Multiply operator.
    Mul,
    /// Power operator.
    Pow,
}

/// Represents a number in Algexenotation.
#[derive(Debug, PartialEq, Clone)]
pub enum Algexeno {
    /// An original natural number.
    Orig(u64),
    /// A constant in Algexenotation.
    Const(u64),
    /// A binary operation.
    Bin(Op, Box<(Algexeno, Algexeno)>),
}

impl fmt::Display for Algexeno {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Orig(x) => write!(w, "{}'", x),
            Const(x) => write!(w, "{}", x),
            Bin(Add, ab) => write!(w, "({}+{})", ab.0, ab.1),
            Bin(Mul, ab) => write!(w, "{}*{}", ab.0, ab.1),
            Bin(Pow, ab) => write!(w, "{}^{}", ab.0, ab.1),
        }
    }
}

impl Algexeno {
    /// Converts to original natural number.
    pub fn original(&self) -> Algexeno {
        match self {
            Orig(x) => Orig(*x),
            Const(x) => Orig(fast_hyperprime_with_lookup(*x).0),
            Bin(Add, ab) => {
                match (&ab.0, &ab.1) {
                    (Const(a), Const(b)) => Const(a + b).original(),
                    (Const(1), b) => {
                        if let Orig(b) = b.original() {
                            Orig(nth_prime_with_lookup(b - 1))
                        } else {unreachable!()}
                    }
                    (Const(a), b) => {
                        Bin(Add, Box::new((Const(1), Bin(Add, Box::new((
                            Const(a - 1),
                            b.clone()
                        )))))).original()
                    }
                    _ => unreachable!("{:?}", self),
                }
            }
            Bin(Mul, ab) => {
                match (ab.0.original(), ab.1.original()) {
                    (Orig(a), Orig(b)) => Orig(a * b),
                    _ => unreachable!(),
                }
            }
            Bin(Pow, ab) => {
                match (ab.0.original(), ab.1.original()) {
                    (Orig(a), Orig(b)) => Orig(a.pow(b as u32)),
                    _ => unreachable!(),
                }
            }
        }
    }

    /// Normalizes to Algexenotation.
    pub fn eval(&self) -> Algexeno {
        match self {
            Const(x) => Const(*x),
            Orig(0) | Orig(1) => self.clone(),
            Orig(2) => Const(0),
            Orig(x) if prime_with_lookup(*x) => {
                Bin(Add, Box::new((
                    Const(1), Orig(count_primes_with_lookup(*x) + 1).eval()
                ))).eval()
            }
            Orig(x) => {
                let mut x = *x;
                let mut i = 2;
                let mut k = 0;
                let mut expr: Option<Algexeno> = None;
                loop {
                    if x % i == 0 {
                        x /= i;
                        k += 1;
                    } else {
                        if k > 0 {
                            let arg = if k == 1 { Orig(i).eval() } else {
                                Bin(Pow, Box::new((
                                    Orig(i).eval(),
                                    Orig(k).eval()
                                ))).eval()
                            };
                            if let Some(left) = expr {
                                expr = Some(Bin(Mul, Box::new((
                                    left, arg
                                ))));
                            } else {
                                expr = Some(arg);
                            }
                        }
                        k = 0;
                        i += 1;

                        if x == 1 {break}
                    }
                }

                if let Some(expr) = expr {expr} else {Orig(x)}
            }
            Bin(Add, ab) => {
                match (&ab.0, &ab.1) {
                    (Const(a), Const(b)) => Const(a + b),
                    (Const(a), Bin(Add, bc)) => {
                        if let Const(b) = bc.0 {
                            Bin(Add, Box::new((
                                Const(a + b), bc.1.clone()
                            ))).eval()
                        } else {
                            self.clone()
                        }
                    }
                    _ => self.clone(),
                }
            }
            Bin(Pow, _) | Bin(Mul, _) => self.clone(),
        }
    }
}

/// Gets the nth prime with lookup table.
pub fn nth_prime_with_lookup(n: u64) -> u64 {
    if let Some(x) = nth_prime_lookup(n) {x}
    else {
        let (mut p, mut i) = last_in_prime_lookup();
        p += 1;
        loop {
            if prime_with_lookup(p) {
                if i == n {return p};
                i += 1;
            }
            p += 1;
        }
    }
}

/// Provides lookup knowledge for primes.
pub fn nth_prime_lookup(n: u64) -> Option<u64> {
    primes::DATA.get(n as usize).map(|n| *n)
}

/// Gets the last prime in lookup table.
pub fn last_in_prime_lookup() -> (u64, u64) {
    (primes::DATA.last().map(|n| *n).unwrap(), primes::DATA.len() as u64 - 1)
}

/// Returns `true` if `n` is a prime.
pub fn prime_with_lookup(n: u64) -> bool {
    if n < 2 {return false};
    let mut i = 0;
    let mut last: Option<u64> = None;
    while let Some(p) = nth_prime_lookup(i) {
        if p == n {return true};
        if n % p == 0 {return false};
        last = Some(p);
        if p > n {return false};
        i += 1;
    }
    if let Some(last) = last {
        for i in last..n {
            if n % i == 0 {return false};
        }
    }
    true
}

/// Returns `true` if `n` is a prime.
pub fn prime(n: u64) -> bool {
    if n < 2 {return false};
    for i in 2..n {
        if n % i == 0 {return false}
    }
    true
}

/// Counts primes with lookup.
pub fn count_primes_with_lookup(x: u64) -> u64 {
    let mut n = 0;
    let mut last: Option<u64> = None;
    while let Some(i) = nth_prime_lookup(n) {
        if i >= x {return n};
        n += 1;
        last = Some(i);
    }
    if let Some(last) = last {
        for i in last + 1..x {
            if prime_with_lookup(i) {n += 1}
        }
    }
    n
}

/// Counts primes below `x`.
pub fn count_primes(x: u64) -> u64 {
    if x <= 2 {return 0};
    let mut n = 0;
    for i in 2..x {
        if prime(i) {n += 1}
    }
    n
}

/// Computes a hyperprime.
pub fn hyperprime(n: u64) -> u64 {
    if n == 0 {2}
    else {
        let p = hyperprime(n - 1) + 1;
        let mut k = 2;
        let mut i = 2;
        loop {
            if prime(k) {
                if i == p {return k};
                i += 1;
            }
            k += 1;
        }
    }
}

/// Provides lookup knowledge of fast hyperprime.
pub fn fast_hyperprime_lookup(n: u64) -> Option<(u64, u64)> {
    match n {
        0 => Some((2, 2)),
        1 => Some((3, 3)),
        2 => Some((5, 4)),
        3 => Some((11, 6)),
        4 => Some((31, 12)),
        5 => Some((127, 32)),
        6 => Some((709, 128)),
        7 => Some((5381, 710)),
        8 => Some((52711, 5382)),
        9 => Some((648391, 52712)),
        10 => Some((9737333, 648392)),
        11 => Some((174440041, 9737334)),
        _ => None,
    }
}

/// Gets fast hyperprime with lookup.
pub fn fast_hyperprime_with_lookup(n: u64) -> (u64, u64) {
    if let Some(x) = fast_hyperprime_lookup(n) {x}
    else {
        let (p0, mut i) = fast_hyperprime_with_lookup(n - 1);
        let p = p0 + 1;
        let mut k = p0;
        loop {
            if prime_with_lookup(k) {
                if i == p {return (k, i)};
                i += 1;
            }
            k += 1;
        }
    }
}

/// Computes a hyperprime.
///
/// Returns an index offset in second component to speed up computation.
pub fn fast_hyperprime(n: u64) -> (u64, u64) {
    if n == 0 {(2, 2)}
    else {
        let (p0, mut i) = fast_hyperprime(n - 1);
        let p = p0 + 1;
        let mut k = p0;
        loop {
            if fermat_prime(k) {
                if i == p {return (k, i)};
                i += 1;
            }
            k += 1;
        }
    }
}

/// Determines whether `p` is a prime using Fermat's primality test.
pub fn fermat_prime(p: u64) -> bool {
    if p < 2 {return false}
    for i in 2..p-1 {
        if !fermat(i, p) {return false}
    }
    true
}

/// Fermat primality set using `a` as witness and `p` as prime.
pub fn fermat(a: u64, p: u64) -> bool {
    (a % p) == 0 || modexp(a, p - 1, p) == 1
}

/// Modulus multiplication.
pub fn modmul(a: u64, b: u64, m: u64) -> u64 {
    ((a % m) * (b % m)) % m
}

/// Modulus exponentiation.
pub fn modexp(a: u64, b: u64, m: u64) -> u64 {
    let mut s = 1;
    let mut b = b;
    let mut base = a % m;
    for _ in 0..b {
        if b % 2 == 1 {
            s = (s * base) % m;
        }
        b = b >> 1;
        base = (base * base) % m;
    }
    s
}

/// Macro for Algexenotation.
///
/// - `x`: hyperprime
/// - `x%`: original natural number `3`
/// - `x^y`: power
/// - `x*y`: multiply
/// - `(x+y)`: hyperprime addition
#[macro_export]
macro_rules! ax {
    ( $x:literal % ) => {Algexeno::Orig($x)};
    ( $x:literal ) => {Algexeno::Const($x)};
    ( $x:ident ) => {Algexeno::Const($x)};
    ( ( $x:tt + $($y:tt)+ ) ) => {Algexeno::Bin(Op::Add, Box::new((ax!($x), ax!($($y)+))))};
    ( $x0:tt $x1:tt + $($y:tt)+ ) => {Algexeno::Bin(Op::Add, Box::new((ax!($x0 $x1), ax!($($y)+))))};
    ( $x:tt + $($y:tt)+ ) => {Algexeno::Bin(Op::Add, Box::new((ax!($x), ax!($($y)+))))};
    ( ( $($x:tt)+ ) * $($y:tt)+ ) => {Algexeno::Bin(Op::Mul, Box::new((ax!($($x)+), ax!($($y)+))))};
    ( $x0:tt ^ $x1:tt * $($y:tt)+ ) => {
        Algexeno::Bin(Op::Mul, Box::new((ax!($x0 ^ $x1), ax!($($y)+))))
    };
    ( $x0:tt * $x1:tt * $($y:tt)+ ) => {
        Algexeno::Bin(Op::Mul, Box::new((ax!($x0 * $x1), ax!($($y)+))))
    };
    ( $x0:tt $x1:tt * $($y:tt)+ ) => {
        Algexeno::Bin(Op::Mul, Box::new((ax!($x0 $x1), ax!($($y)+))))
    };
    ( $x:tt * $($y:tt)+ ) => {Algexeno::Bin(Op::Mul, Box::new((ax!($x), ax!($($y)+))))};
    ( $x:tt ^ ( $($y:tt)+ ) ) => {Algexeno::Bin(Op::Pow, Box::new((ax!($x), ax!($($y)+))))};
    ( $x0:tt $x1:tt ^ $($y:tt)+ ) => {Algexeno::Bin(Op::Pow, Box::new((ax!($x0 $x1), ax!($($y)+))))};
    ( $x:tt ^ $($y:tt)+ ) => {Algexeno::Bin(Op::Pow, Box::new((ax!($x), ax!($($y)+))))};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_hyperprime() {
        assert_eq!(hyperprime(0), 2);
        assert_eq!(hyperprime(1), 3);
        assert_eq!(hyperprime(2), 5);
        assert_eq!(hyperprime(3), 11);
        assert_eq!(hyperprime(4), 31);

        assert_eq!(fast_hyperprime(0), (2, 2));
        assert_eq!(fast_hyperprime(1), (3, 3));
        assert_eq!(fast_hyperprime(2), (5, 4));
        assert_eq!(fast_hyperprime(3), (11, 6));
        assert_eq!(fast_hyperprime(4), (31, 12));
    }

    #[test]
    fn test_original() {
        let a = Const(0);
        assert_eq!(a.original(), Orig(2));

        let a = Const(1);
        assert_eq!(a.original(), Orig(3));

        let a = Const(2);
        assert_eq!(a.original(), Orig(5));

        let a = Const(3);
        assert_eq!(a.original(), Orig(11));

        let a = Const(4);
        assert_eq!(a.original(), Orig(31));

        let a = Bin(Add, Box::new((Const(1), Const(1))));
        assert_eq!(a.original(), Orig(5));

        let a = Bin(Mul, Box::new((Const(0), Const(1))));
        assert_eq!(a.original(), Orig(6));

        let a = Bin(Pow, Box::new((Const(0), Const(1))));
        assert_eq!(a.original(), Orig(8));

        let a = Bin(Add, Box::new((
            Const(1),
            Bin(Mul, Box::new((Const(0), Const(1))))
        )));
        assert_eq!(a.original(), Orig(13));
    }

    #[test]
    fn test_eval() {
        let a = Orig(2);
        assert_eq!(a.eval(), Const(0));

        let a = Orig(3);
        assert_eq!(a.eval(), Const(1));

        let a = Orig(4);
        assert_eq!(a.eval(), Bin(Pow, Box::new((Const(0), Const(0)))));

        let a = Orig(5);
        assert_eq!(a.eval(), Const(2));

        let a = Orig(6);
        assert_eq!(a.eval(), Bin(Mul, Box::new((Const(0), Const(1)))));

        let a = Orig(7);
        assert_eq!(a.eval(), Bin(Add, Box::new((Const(1), Orig(4).eval()))));

        let a = Orig(8);
        assert_eq!(a.eval(), Bin(Pow, Box::new((Const(0), Const(1)))));

        let a = Orig(9);
        assert_eq!(a.eval(), Bin(Pow, Box::new((Const(1), Const(0)))));

        let a = Orig(10);
        assert_eq!(a.eval(), Bin(Mul, Box::new((Const(0), Const(2)))));

        let a = Orig(11);
        assert_eq!(a.eval(), Const(3));

        let a = Orig(12);
        assert_eq!(a.eval(), Bin(Mul, Box::new((
            Orig(4).eval(),
            Const(1)
        ))));

        let a = Orig(13);
        assert_eq!(a.eval(), Bin(Add, Box::new((
            Const(1),
            Bin(Mul, Box::new((Const(0), Const(1))))
        ))));

        let a = Orig(14);
        assert_eq!(a.eval(), Bin(Mul, Box::new((Const(0), Orig(7).eval()))));

        let a = Orig(15);
        assert_eq!(a.eval(), Bin(Mul, Box::new((Const(1), Const(2)))));

        let a = Orig(16);
        assert_eq!(a.eval(), Bin(Pow, Box::new((Const(0), Orig(4).eval()))));

        let a = Orig(17);
        assert_eq!(a.eval(), Bin(Add, Box::new((Const(2), Orig(4).eval()))));

        let a = Orig(18);
        assert_eq!(a.eval(), Bin(Mul, Box::new((Const(0), Orig(9).eval()))));

        let a = Orig(31);
        assert_eq!(a.eval(), Const(4));
    }

    #[test]
    fn test_iso() {
        for i in 0..100 {
            assert_eq!(Orig(i).eval().original(), Orig(i));
        }
    }

    #[test]
    fn test_macro() {
        let a = ax!(0);
        assert_eq!(a, Const(0));

        let a = ax!(3);
        assert_eq!(a, Const(3));

        let a = ax!(3%);
        assert_eq!(a, Orig(3));

        let a = ax!(1 + 0);
        assert_eq!(a, Bin(Add, Box::new((Const(1), Const(0)))));

        let a = ax!(0 * 1);
        assert_eq!(a, Bin(Mul, Box::new((Const(0), Const(1)))));

        let a = ax!(0^0);
        assert_eq!(a, Bin(Pow, Box::new((Const(0), Const(0)))));

        let a = ax!((1 + 0^0) * 2);
        assert_eq!(a, Bin(Mul, Box::new((
            Bin(Add, Box::new((Const(1), Orig(4).eval()))),
            Const(2)
        ))));

        assert_eq!(ax!(0^(0^0)).original(), Orig(16));
    }

    #[test]
    fn test_count() {
        assert_eq!(count_primes(2), 0);

        for i in 0..100 {
            assert_eq!(count_primes(i), count_primes_with_lookup(i));
        }
    }

    #[test]
    fn test_prime() {
        for i in 0..100 {
            println!("{}", i);
            assert_eq!(prime(i), prime_with_lookup(i));
        }
    }

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime_with_lookup(0), 2);
        assert_eq!(nth_prime_with_lookup(1), 3);
        assert_eq!(nth_prime_with_lookup(2), 5);
        assert_eq!(nth_prime_with_lookup(3), 7);
        assert_eq!(nth_prime_with_lookup(4), 11);
    }
}
