//https://atcoder.jp/contests/typical90/tasks/typical90_t

use proconio::input;

pub fn solve() {
    input! {
        a: u64,
        b: u32,
        c: u64,
    }
    let r = c.pow(b);
    if a < r {
        println!("Yes");
    } else {
        println!("No");
    }
}
