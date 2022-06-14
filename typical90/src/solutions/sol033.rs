// https://atcoder.jp/contests/typical90/tasks/typical90_ag

use proconio::input;

pub fn solve() {
    input! {
        h: u8,
        w: u8,
    }
    
    if h == 1 {
        println!("{}", w);
        return;
    } else if w == 1 {
        println!("{}", h);
        return;
    }

    let mut counter = 0;
    for i in 0..h {
        for j in 0..w {
            if i % 2 == 0 && j % 2 == 0 { counter += 1; }
        }
    }
    println!("{}", counter);
}