// https://atcoder.jp/contests/typical90/tasks/typical90_x

use proconio::input;

pub fn solve() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    }
    let mut counter = 0;

    for i in 0..n {
        counter += (b[i] - a[i]).abs();
    }

    if k >= counter && (k - counter) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}