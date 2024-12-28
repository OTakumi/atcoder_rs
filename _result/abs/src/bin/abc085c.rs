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
        n: u64,
        y: u64,
    }

    for ten in (0..=n).rev() {
        for five in (0..=(n - ten)).rev() {
            let one = n - ten - five;
            if ten * 10000 + five * 5000 + one * 1000 == y {
                println!("{} {} {}", ten, five, one);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
