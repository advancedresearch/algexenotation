/*

This example shows that a triangle is `17719'`.

```
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

*/

use algexenotation::*;

fn main() {
    let n = ax!((1+0*1) * (1+1*2) * (1+2*0));
    // Prints `17719'`.
    println!("{}", n.original());
}
