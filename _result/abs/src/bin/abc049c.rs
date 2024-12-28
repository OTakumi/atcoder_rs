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
        mut s: String,
    }

    let s = s.replace("eraser", "");
    let s = s.replace("erase", "");
    let s = s.replace("dreamer", "");
    let s = s.replace("dream", "");

    if s.is_empty() {
        println!("YES");
    } else {
        println!("NO");
    }
}
