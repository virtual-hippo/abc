// https://atcoder.jp/contests/typical90/tasks/typical90_v

use proconio::input;

fn gcd<T>(a: T, b: T) -> T
    where T: std::ops::Rem<Output = T>,
          T: std::cmp::PartialEq,
          T: std::convert::From<u8>,
          T: Copy
{
    if b % a == 0_u8.into() { a }
    else {
        gcd(b % a, a)
    }
}

pub fn solve() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    let r = gcd(a, gcd(b, c));
    println!("{}", (a/r-1)+(b/r-1)+(c/r-1));
}

pub fn solve_162_c() {
    input! {
        k: u16
    }
    let mut res: u64 = 0;
    for ii in 1..=k {
        for jj in 1..=k {
            for kk in 1..=k {
                res += gcd(ii as u64,gcd(jj as u64,kk as u64));
            }
        }   
    }
    println!("{}", res);
}