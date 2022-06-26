// https://atcoder.jp/contests/typical90/tasks/typical90_g

use proconio::input;
use std::io::{stdout, Write, BufWriter};

fn lower_bound(v: i64, list: &Vec<i64>) -> usize {
    let mut l = 0;
    let mut r = list.len() - 1;
    let mut res = 0;

    while l <= r {
        let mid = (l + r) / 2;
        if list[mid] == v {
            res = mid;
            break;
        } else if list[mid] < v {
            l = mid + 1;
            res = l;
        } else if mid != 0 {
            r = mid - 1;
            res = r;
        } else {
            res = 0;
            break;
        }
    }
    if res > list.len() - 1 { list.len() }
    else if list[res] >= v && res != 0 { res - 1 }
    else { res }
}

pub fn solve() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        b: [i64; q],
    }
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    a.sort_unstable();
    for ele in b.iter() {
        let pos = lower_bound(*ele, &a);
        let ans = if b.len() == 1 {
            (*ele as i64 - a[0] as i64).abs()
        } else if pos >= n - 1 {
            (*ele as i64 - a[n - 1] as i64).abs()
        } else {
            std::cmp::min((a[pos + 1]  as i64 - *ele).abs(), (a[pos] as i64 - *ele).abs())
        };

        writeln!(out, "{}", ans).unwrap();
    }
}