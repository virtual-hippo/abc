// https://atcoder.jp/contests/typical90/tasks/typical90_al

use proconio::input;

fn gcd(v1: u64, v2: u64) -> u64 {
    if v2 == 0 { v1 }
    else { gcd(v2, v1 % v2) }
}

pub fn solve() {
    input! {
        a: u64,
        b: u64,
    }
    let common_divisor = gcd(a, b);
    let r = b/common_divisor;
    if r > 1_000_000_000_000_000_000 / a {
        println!("Large")
    } else {
        println!("{}", a * r)
    }
}