// https://atcoder.jp/contests/typical90/tasks/typical90_bc

use proconio::input;

pub fn solve() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n],
    }
    let mut counter = 0;
    for i in 0..n-4 {
        for j in i+1..n-3 {
            for k in j+1..n-2 {
                for l in k+1..n-1 {
                    for m in l+1..n {
                        if (a[i] % p * a[j] % p * a[k] % p * a[l] % p * a[m]  % p) == q {counter += 1}
                    }
                }
            }
        }
    }
    println!("{}", counter);
}
