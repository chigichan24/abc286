use proconio::*;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let from = "na";
    let to = "nya";
    let result = s.replace(&from, to);
    println!("{}", result);
}
