// https://atcoder.jp/contests/typical90/tasks/typical90_bz

use proconio::input;

pub fn solve() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut table = vec![0; n];
    for (a, b) in ab.iter() {
        if *a > *b { table[*a - 1] += 1; }
        else  { table[*b - 1] += 1; }
    }
    let mut ans = 0;
    for v in table.iter() {
        if *v == 1 { ans += 1; }
    }
    println!("{}", ans);
}