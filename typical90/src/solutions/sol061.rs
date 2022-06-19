// https://atcoder.jp/contests/typical90/tasks/typical90_bi

use proconio::input;
use std::collections::VecDeque;

pub fn solve() {
    input! {
        q: usize,
        tx: [(usize, usize); q],
    }
    let mut deque: VecDeque<usize> = VecDeque::new();

    for (t, x) in tx.iter() {
        match *t {
            1 => deque.push_front(*x),
            2 => deque.push_back(*x),
            3 => println!("{}", deque[*x - 1]),
            _ => println!("unexpected")
        }
    }
}