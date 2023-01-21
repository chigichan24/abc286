use std::mem;

use proconio::*;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        mut a: [usize; n],
    }
    for i in (p-1)..q {
        a.swap(i,i+r-p);
    }
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!("");
}
