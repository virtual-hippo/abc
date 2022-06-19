// https://atcoder.jp/contests/typical90/tasks/typical90_bo

use proconio::input;

pub fn solve() {
    input! {
        mut n: String,
        k: usize,
    }

    for _ in 0..k {
        let mut n_u64;
        match u64::from_str_radix(&n, 8) {
            Ok(v) => n_u64 = v,
            Err(_err) => {
                n = "0".to_string();
                break;
            },
        }
        let mut n_9 = "".to_string();
        while n_u64 != 0 {
            n_9.push(std::char::from_digit((n_u64 % 9) as u32, 10).unwrap());
            n_u64 /= 9;
        }
        n = n_9.chars().rev().map(|x| if x == '8' {'5'} else {x}).collect();
    }
    println!("{}", n);
}