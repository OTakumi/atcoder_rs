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
        mut input: [(i64, i64, i64); n],
    }

    let mut prev = (0, 0, 0);

    // Not feasible if coordinate distance is greater than time difference
    // or if time difference is odd and coordinate distance is even
    for (cur_t, cur_x, cur_y) in input {
        let (pt, px, py) = prev;
        let d = (cur_x - px).abs() + (cur_y - py).abs();

        let dt = cur_t - pt;

        if dt < d {
            println!("No");
            return;
        }

        if (dt - d) % 2 != 0 {
            println!("No");
            return;
        }

        prev = (cur_t, cur_x, cur_y);
    }

    println!("Yes");
}
