// https://atcoder.jp/contests/typical90/tasks/typical90_p

use proconio::input;
use std::cmp;

pub fn solve() {
    input! {
        n: u64,
        a: u64,
        b: u64,
        c: u64,
    }
    
    let mut ans = 9999;
    for ai in 0..9999 {
        for bi in 0..9999 {
            if n >= a * ai + b * bi &&
                (n - a * ai - b * bi) % c == 0
            {
                ans = cmp::min(
                    ans,
                    (n - a * ai - b * bi) / c + ai + bi
                );
            }
        }
    }
    println!("{}", ans);
}
