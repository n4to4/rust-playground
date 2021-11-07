#![feature(generic_const_exprs)]
#![allow(dead_code, unused_variables)]

// https://nora.codes/post/its-time-to-get-hyped-about-const-generics-in-rust/

struct Assert<const COND: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}

#[derive(Debug, Clone)]
struct Foo<const N: usize> {
    inner: [usize; N],
}

impl<const N: usize> Copy for Foo<N> where Assert<{ N < 128 }>: IsTrue {}

fn is_copy<T: Copy>(_: &T) {}

fn main() {
    let ok = Foo::<16> { inner: [0; 16] };
    is_copy(&ok);

    let bad = Foo::<128> { inner: [0; 128] };
    //is_copy(&bad);

    // error[E0308]: mismatched types
    //   --> src/main.rs:25:5
    //    |
    // 25 |     is_copy(&bad); // doesn't compile
    //    |     ^^^^^^^ expected `false`, found `true`
    //    |
    //    = note: expected type `false`
    //               found type `true`
}
