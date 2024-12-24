use proconio::input;

fn main() {
    input! {
        n: u32,
        mut an: [i64; n],      // 'an' is Vec<u64>
    }

    let mut cnt = 0;

    while an.iter().all(|&a| a % 2 == 0) {
        for a in an.iter_mut() {
            *a /= 2;
        }
        cnt += 1;
    }

    println!("{}", cnt)
}
