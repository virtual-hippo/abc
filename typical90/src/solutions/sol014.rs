// https://atcoder.jp/contests/typical90/tasks/typical90_n

use proconio::input;

pub fn solve() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }
    a.sort_unstable();
    b.sort_unstable();

    let mut ans = 0;
    for i in 0..n {
        ans += (a[i] - b[i]).abs();
    }
    println!("{}", ans);
}