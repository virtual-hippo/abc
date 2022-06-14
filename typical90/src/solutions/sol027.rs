// https://atcoder.jp/contests/typical90/tasks/typical90_aa

use proconio::input;
use std::collections::HashMap;

pub fn solve() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut user = HashMap::<String, i32>::new();
    
    for i in 0..n {
        *user.entry(s[i].clone()).or_insert(0) += 1;
        if user[&s[i]] == 1 {println!("{}", i + 1);}
    }
}