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
        a: i64,
        b: i64,
        c: i64,
        x: i64,
    }

    let mut rslt = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if (500 * i) + (100 * j) + (50 * k) == x {
                    rslt += 1;
                }
            }
        }
    }

    println!("{rslt}");
}
