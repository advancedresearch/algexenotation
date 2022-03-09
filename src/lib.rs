//! # Algexenotation
//!
//! Algexenotation is a way to represent multisets
//! as natural numbers with algebraic compression.
//! Inspired by [Tic Xenotation](https://mvupress.net/tic-xenotation/).
//! For more information, see [paper](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/algexenotation.pdf).

#![deny(missing_docs)]

use Algexeno::*;
use Op::*;

use std::fmt;

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
            Const(x) => Orig(hyperprime(*x)),
            Bin(Add, ab) => {
                match (&ab.0, &ab.1) {
                    (Const(a), Const(b)) => Const(a + b).original(),
                    (Const(1), b) => {
                        if let Orig(b) = b.original() {
                            let mut p = 2;
                            let mut i = 0;
                            loop {
                                if prime(p) {
                                    i += 1;
                                    if i == b {break}
                                }
                                p += 1;
                            }
                            Orig(p)
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
            Orig(x) if prime(*x) => {
                Bin(Add, Box::new((
                    Const(1), Orig(count_primes(*x)).eval()
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

fn prime(n: u64) -> bool {
    if n < 2 {return false};
    for i in 2..n {
        if n % i == 0 {return false}
    }
    true
}

fn count_primes(x: u64) -> u64 {
    let mut n = 1;
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
}
