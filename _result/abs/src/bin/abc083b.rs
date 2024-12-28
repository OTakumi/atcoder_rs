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
        n: u32,
        a: u32,
        b: u32,
    }

    let mut rslt = 0;

    for i in 0..=n {
        let mut x = i;
        let mut sum = 0;

        while x > 0 {
            sum += x % 10;
            x /= 10;
        }

        if a <= sum && sum <= b {
            rslt += i;
        }
    }

    println!("{rslt}");
}
