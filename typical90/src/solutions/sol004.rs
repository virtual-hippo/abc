// https://atcoder.jp/contests/typical90/tasks/typical90_d

use proconio::input;

pub fn solve() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h],
    }

    let mut h_vec:Vec<u64> = Vec::new();
    let mut w_vec:Vec<u64> = Vec::new();
    for i in 0..h {
        h_vec.push(a[i].iter().fold(0, |sum, e| sum + e));
    }

    for j in 0..w {
        w_vec.push(a.iter().fold(0, |sum, e| sum + e[j]));
    }

    for i in 0..h {
        for j in 0..w {
            let res = h_vec[i] + w_vec[j] - a[i][j];
            print!("{} ", res);
        }
        print!("\n");
    }
}