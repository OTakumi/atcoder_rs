use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let rslt = if a * b % 2 == 0 { "Even" } else { "Odd" };

    println!("{}", rslt);
}
