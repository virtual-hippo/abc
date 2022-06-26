// https://atcoder.jp/contests/typical90/tasks/typical90_b

use proconio::input;

fn is_correct(s: String) -> bool {
    let mut cnt_l = 0;
    let mut cnt_r = 0;
    for ch in s.chars() {
        if ch == '(' { cnt_l += 1; }
        else if ch == ')' { cnt_r += 1; }
        if cnt_l < cnt_r { return false;}
    }
    cnt_l == cnt_r
}

pub fn solve() {
    input! {
        n: usize,
    }
    for i in 0..(1 << n) {
        let mut candidate = "".to_string();
        for j in (0..=n-1).rev() {
            if i & (1 << j) == 0{
                candidate.push('(');
            } else {
                candidate.push(')');
            }
        }
        if is_correct(candidate.clone()) {println!("{}", &candidate)}
    }
}