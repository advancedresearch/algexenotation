# Algexenotation

Algexenotation is a way to represent multisets
as natural numbers with algebraic compression.
Inspired by [Tic Xenotation](https://mvupress.net/tic-xenotation/).
For more information, see [paper](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/algexenotation.pdf).

### Motivation

- Model ordered Cartesian combinatorics
- Algebraic compression of Tic Xenotation
- Encoding hypergraphs as natural numbers

### Triangle example

This example shows that a triangle is `17719'`.

```text
               0
               o
              / \
       1+0*1 /   \ 1+2*0
            /     \
         1 o-------o 2
             1+1*2
```

There are 3 nodes `0, 1, 2` in the triangle.
In Algexenotation, these are hyperprimes.

An edge from `a` to `b` is encoded `1+a*b`.

The product of the 3 edges is the triangle.

```rust
use algexenotation::*;

fn main() {
    let n = ax!((1+0*1) * (1+1*2) * (1+2*0));
    // Prints `17719'`.
    println!("{}", n.original());
}
```

### Introduction to Algexenotation

Algexenotation might be thought of as a generalization of prime factorization.

#### Original Numbers

An "original number" in Algexenotation is what we think of as natural number
written in the usual form.
For example, "four" is written `4'`, with a "'".

#### Power

When evaluating `4'`, we get `0^0`.
Here, `0` is not an original number, but the 0th prime which is `2'`.

The power operator `^` is interpreted in the usual way,
such that `0^0` is the same as `2'^2'`.

#### Multiplication

In Algexenotation, all original numbers except `0'` and `1'` get evaluated to another form.
This process of evaluation corresponds to prime factorization.

For example, `6'` gets evaluated to `0*1`.
Here, `0 = 2'` and `1 = 3'`.

The multiplication operator `*` is interpreted in the usual way.

#### Addition

What makes Algexenotation different from the usual notation,
is that addition means something entirely different.

For example, `2' + 3'` does not evaluate to `5'`.
Instead, since `0 = 2'` and `1 = 3'`,
`2' + 3'` evaluates to `0 + 1 = 1`.

It takes a while to get used to this way of thinking about addition.
If you don't get it at first, then don't panic!
Algexenotation can be mind boggling sometimes.

However, when you add two simple Algexenic numbers,
for example `6 + 7 = 13`, you can just compute as normal.

#### Hyperprime

The reason addition works the way it does in Algexenotation,
is due to "hyperprimes".

Hyperprimes are written `0, 1, 2, 3, ...` in Algexenotation.

The smallest hyperprime is `0 = 2'`,
because it is the 0th prime in the prime sequence of natural numbers.

The next hyperprime is `1 = 3'`,
because it is the `2'`nd prime (or `0`th prime).

The next hyperprime is `2 = 5'`,
because it is the `3'`rd prime (or `1`th prime).

The next hyperprime is `3 = 11'`,
because it is the `5'`th prime (or `2`nd prime).

The next hyperprime is `4 = 31'`,
because it is the `11'`th prime (or `3`rd prime).

The next hyperprime is `5 = 127'`,
because it is the `31'`th prime (or `4`th prime).

Notice that the next hyperprime refers to the previous hyperprime-th prime
in the prime sequence.

This sequence is a sub-sequence of the prime sequence,
but grows much more rapidly.

#### Seven

The number `7'` is the smallest "addition" number in Algexenotation:

```text
1+0^0 = 7'
```

It means that one must use `+` to express `7'`.

Now, since `0^0 = 4'` and `4'` is between `1 = 3'` and `2 = 5'`,
it seems kind of intuitive that `3'+4'=7'`.
However, that is wrong.

#### Thirteen

The number `13'` is the second "addition" number in Algexenotation:

```text
1+0*1 = 13'
```

If we interpret `+` in the usual way,
then we get `3'+2'*3' = 9'`, which is wrong.

Instead, `+` must be thought of as a different kind of addition than in the usual sense.
It works normally for hyperprimes, but for other numbers it is harder to understand.

The correct way to interpret `1+0*1`,
is by thinking of the `6'`th prime (or `0*1`th prime).

### Fourteen

The first composite number with two different prime bases is `14'`.

```text
0*(1+0^0) = 14'
```

This is intuitive, since `0 = 2'` and `1+0^0 = 7'`.

### Seventeen

The first "addition" number using `2` in Algexenotation is `17'`.

```text
2+0^0 = 17'
```

The way to interpret this correctly,
is as `1+(1+0^0)`, where `1+0^0 = 7'`,
so one gets the `7'`th prime.# Algexenotation

Algexenotation is a way to represent multisets
as natural numbers with algebraic compression.
Inspired by [Tic Xenotation](https://mvupress.net/tic-xenotation/).
For more information, see [paper](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/algexenotation.pdf).

### Motivation

- Model ordered Cartesian combinatorics
- Algebraic compression of Tic Xenotation
- Encoding hypergraphs as natural numbers

### Triangle example

This example shows that a triangle is `17719'`.

```text
               0
               o
              / \
       1+0*1 /   \ 1+2*0
            /     \
         1 o-------o 2
             1+1*2
```

There are 3 nodes `0, 1, 2` in the triangle.
In Algexenotation, these are hyperprimes.

An edge from `a` to `b` is encoded `1+a*b`.

The product of the 3 edges is the triangle.

```rust
use algexenotation::*;

fn main() {
    let n = ax!((1+0*1) * (1+1*2) * (1+2*0));
    // Prints `17719'`.
    println!("{}", n.original());
}
```

### Introduction to Algexenotation

Algexenotation might be thought of as a generalization of prime factorization.

#### Original Numbers

An "original number" in Algexenotation is what we think of as natural number
written in the usual form.
For example, "four" is written `4'`, with a "'".

#### Power

When evaluating `4'`, we get `0^0`.
Here, `0` is not an original number, but the 0th prime which is `2'`.

The power operator `^` is interpreted in the usual way,
such that `0^0` is the same as `2'^2'`.

#### Multiplication

In Algexenotation, all original numbers except `0'` and `1'` get evaluated to another form.
This process of evaluation corresponds to prime factorization.

For example, `6'` gets evaluated to `0*1`.
Here, `0 = 2'` and `1 = 3'`.

The multiplication operator `*` is interpreted in the usual way.

#### Addition

What makes Algexenotation different from the usual notation,
is that addition means something entirely different.

For example, `2' + 3'` does not evaluate to `5'`.
Instead, since `0 = 2'` and `1 = 3'`,
`2' + 3'` evaluates to `0 + 1 = 1`.

It takes a while to get used to this way of thinking about addition.
If you don't get it at first, then don't panic!
Algexenotation can be mind boggling sometimes.

However, when you add two simple Algexenic numbers,
for example `6 + 7 = 13`, you can just compute as normal.

#### Hyperprime

The reason addition works the way it does in Algexenotation,
is due to "hyperprimes".

Hyperprimes are written `0, 1, 2, 3, ...` in Algexenotation.

The smallest hyperprime is `0 = 2'`,
because it is the 0th prime in the prime sequence of natural numbers.

The next hyperprime is `1 = 3'`,
because it is the `2'`nd prime (or `0`th prime).

The next hyperprime is `2 = 5'`,
because it is the `3'`rd prime (or `1`th prime).

The next hyperprime is `3 = 11'`,
because it is the `5'`th prime (or `2`nd prime).

The next hyperprime is `4 = 31'`,
because it is the `11'`th prime (or `3`rd prime).

The next hyperprime is `5 = 127'`,
because it is the `31'`th prime (or `4`th prime).

Notice that the next hyperprime refers to the previous hyperprime-th prime
in the prime sequence.

This sequence is a sub-sequence of the prime sequence,
but grows much more rapidly.

#### Seven

The number `7'` is the smallest "addition" number in Algexenotation:

```text
1+0^0 = 7'
```

It means that one must use `+` to express `7'`.

Now, since `0^0 = 4'` and `4'` is between `1 = 3'` and `2 = 5'`,
it seems kind of intuitive that `3'+4'=7'`.
However, that is wrong.

#### Thirteen

The number `13'` is the second "addition" number in Algexenotation:

```text
1+0*1 = 13'
```

If we interpret `+` in the usual way,
then we get `3'+2'*3' = 9'`, which is wrong.

Instead, `+` must be thought of as a different kind of addition than in the usual sense.
It works normally for hyperprimes, but for other numbers it is harder to understand.

The correct way to interpret `1+0*1`,
is by thinking of the `6'`th prime (or `0*1`th prime).

### Fourteen

The first composite number with two different prime bases is `14'`.

```text
0*(1+0^0) = 14'
```

This is intuitive, since `0 = 2'` and `1+0^0 = 7'`.

### Seventeen

The first "addition" number using `2` in Algexenotation is `17'`.

```text
2+0^0 = 17'
```

The way to interpret this correctly,
is as `1+(1+0^0)`, where `1+0^0 = 7'`,
so one gets the `7'`th prime.
