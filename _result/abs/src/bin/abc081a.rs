use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let rslt = s.chars().filter(|&c| c == '1').count();
    println!("{}", rslt);
}
