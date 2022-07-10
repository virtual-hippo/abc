// https://atcoder.jp/contests/typical90/tasks/typical90_r

use proconio::input;
use std::f64::consts::PI;


pub fn solve() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q],
    }

    for v in e.iter() {
        
        let radian = -(2.0 * PI * (*v / t)) - 0.5 * PI;

        let point3d = (
            0.0,
            (l * 0.5 * radian.cos()),
            l * 0.5 * radian.sin() + l * 0.5
        );

        let chokudai = (x, y, 0.0);

        let d = ((chokudai.0 - point3d.0).powi(2) +
                (chokudai.1 - point3d.1).powi(2) +
                (chokudai.2 - point3d.2).powi(2)).sqrt();

        println!("{}", (point3d.2 / d).asin() * (180.0 / PI));
    }
    
}