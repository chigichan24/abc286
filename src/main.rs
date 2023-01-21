use std::i64;
use proconio::*;

fn calc_change_count_for_palindrome_string(target: &str) -> i64 {
    let length = target.len() / 2;
    let mut cnt = 0;
    for i in 0..length {
        if target.get(i..i+1) != target.get(target.len() - 1 - i..target.len() - i) {
            cnt += 1;
        }
    }
    return cnt;
}

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        mut s: String,
    }
    let mut result: i64 = i64::MAX;

    for i in 0..=n {
        let cost_a: i64 = i * a;
        let cost_b: i64 = calc_change_count_for_palindrome_string(&s) * b;
        result = result.min(cost_a + cost_b);
        s.push(s.chars().nth(0).unwrap());
        s.remove(0);
    }
    println!("{}", result);
}
