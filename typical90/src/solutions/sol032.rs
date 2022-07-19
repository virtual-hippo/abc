// https://atcoder.jp/contests/typical90/tasks/typical90_af

use proconio::input;
use itertools::Itertools;

pub fn solve() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        m: usize,
        xy:[(usize, usize); m],
    }

    let mut kenaku: Vec<Vec<usize>> = vec![vec![]; n];

    for v in xy.iter() {
        kenaku[v.0 - 1].push(v.1 - 1);
        kenaku[v.1 - 1].push(v.0 - 1);
    }

    let mut min = 100_000;

    for perm in (0..n).permutations(n) {
        let mut sum = 0;
        let mut is_kenaku = false;
        for (i, j) in perm.iter().enumerate() {
            sum += a[*j][i];
            if i < n - 1 && kenaku[*j].iter().any(|&v| v==perm[i + 1]) {
                is_kenaku = true;
                break;
            }
        }
        if is_kenaku == false {
            min = std::cmp::min(min, sum);
        }
    }
    if min == 100_000 {
        println!("-1");
    } else {
        println!("{}", min);
    }
    
}