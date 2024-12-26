#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    let mut sum = 0;
    // Descending sort
    a.sort_by(|a, b| a.cmp(b).reverse());

    for (i, v) in a.iter().enumerate().take(n) {
        if i % 2 == 0 {
            sum += v;
        } else {
            sum -= v;
        }
    }

    println!("{sum}");
}
