// https://atcoder.jp/contests/typical90/tasks/typical90_j

use proconio::input;

pub fn solve() {
    input! {
        n: usize,
        cp: [(u8, u64); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut sum_vec1: Vec<u64> = vec![0];
    let mut sum_vec2: Vec<u64> = vec![0];

    for i in 0..n {
        
        if cp[i].0 == 1 {
            sum_vec1.push(sum_vec1[i] + cp[i].1);
            sum_vec2.push(sum_vec2[i] + 0);
        } else {
            sum_vec1.push(sum_vec1[i] + 0);
            sum_vec2.push(sum_vec2[i] + cp[i].1);
        }
        
    }

    for (l, r) in lr.iter() {
        println!("{} {}", sum_vec1[*r] - sum_vec1[*l - 1], sum_vec2[*r] - sum_vec2[*l - 1]);
    }
}