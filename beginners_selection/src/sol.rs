use proconio::input;

pub fn call_solve1() {
    input! {
        a: i32,
        b: i32,
    }
    if (a * b) % 2 == 0 {
        println!("Even");
    } else {
        println!("odd");
    }
}

pub fn call_solve2() {
    input! {
        s: String
    }
    let mut counter = 0;
    if s.chars().nth(0).unwrap() == '1' { counter += 1 }
    if s.chars().nth(1).unwrap() == '1' { counter += 1 }
    if s.chars().nth(2).unwrap() == '1' { counter += 1 }
    println!("{}", counter)
}

pub fn call_solve3() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut min = -1;
    for i in a {
        let mut counter = 0;
        let mut target = i;
        loop {
            if target % 2 == 1 || (min != -1 && counter >= min) { break;}
            target /= 2;
            counter += 1;
        }
        if counter < min || min == -1  {min = counter}
    }
    println!("{}", min)
}

pub fn call_solve4() {
    input! {
        a: usize,
      	b: usize,
      	c: usize,
      	x: usize,
    }
    let mut counter = 0;
  	let mut i_a = 0;
    while i_a < a + 1 {
        let mut i_b = 0;
        while i_b < b + 1 {
            let mut i_c = 0;
            while i_c < c + 1 {
                if (500 * i_a + 100 * i_b + 50 * i_c) == x {counter+=1}
                i_c += 1;
    		}
            i_b += 1;
    	}
        i_a += 1;
    }
    println!("{}", counter)
}

pub fn call_solve5() {
    input! {
        n: u64,
        a: u64,
        b: u64,
    }
    
    let mut total = 0;

    for i in 1..=n {
        let sum = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .fold(0, |sum, i| sum + i);
 
        if a <= sum && sum <= b {
            total += i;
        }
    }
    println!("{}", total)
}

pub fn call_solve6() {
    input! {
        n: u64,
        mut a: [u64; n],
    }
    
    a.sort_unstable_by(|x, y| y.cmp(x));
    let mut alice = 0;
    let mut bob = 0;
    for (i, v) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += v;
        } else {
            bob += v;
        }
    }
    println!("{}", alice - bob)
}

pub fn call_solve7() {
    use std::collections::HashSet;
    input! {
        n: u16,
        d: [u16; n],
    }
    let mut set: HashSet<u16> = HashSet::new();

    for v in d {
        set.insert(v);
    }
    println!("{}", set.len())
}

pub fn call_solve8() {
    input! {
        n: i32,
        y: i32,
    }
    let mut answer = (-1,-1,-1);
    for a in 0..n+1 {
        for b in 0..n+1 {
            if 10000 * a + 5000 * b + 1000 * (n - a - b) == y  && n >= (a + b){
                answer = (a, b, n - a - b);
                break;
            }
        }
    }

    println!("{} {} {}", answer.0, answer.1, answer.2)
}

pub fn call_solve9() {
    input! {
        s: String,
    }

    #[derive(PartialEq)] 
    enum Mode {
        Dream,
        Dreamer,
        Erase,
        Eraser,
        Initial,
    }
    let mut mode = Mode::Initial;
    let mut buf: String = "".to_string();
    let mut status = false;

    for c in s.as_str().chars() {
        status = false;
        if mode == Mode::Initial {
            if c == 'd' {
                mode =  Mode::Dream;
                buf.push(c);
            } else if c == 'e' {
                mode =  Mode::Erase;
                buf.push(c);
            } else {
                break;
            }
        } else if mode == Mode::Dream {
            buf.push(c);
            if buf == "dr" || buf == "dre" || buf == "drea" {
                continue;
            } else if buf == "dream" {
                mode = Mode::Dreamer;
                status = true;
                continue;
            } else {
                break;
            }
        } else if mode == Mode::Erase {
            buf.push(c);
            if buf == "er" || buf == "era" || buf == "eras" {
                continue;
            } else if buf == "erase" {
                mode = Mode::Eraser;
                status = true;
                continue;
            } else {
                break;
            }
        } else if mode == Mode::Dreamer {
            buf.push(c);
            if buf == "dreame"{
                continue;
            } else if buf == "dreamer" {
                status = true;
                continue;
            } else if buf == "dreamerd" ||  buf == "dreamd" {
                mode = Mode::Dream;
                buf = "d".to_string();
                continue;
            } else if buf == "dreamere" ||  buf == "dreame"  {
                mode = Mode::Erase;
                buf = "e".to_string();
                continue;
            } else if buf == "dreamera" {
                mode = Mode::Erase;
                buf = "era".to_string();
                continue;
            } else {
                break;
            }
        } else if mode == Mode::Eraser {
            buf.push(c);
            if buf == "eraser" {
                status = true;
                continue;
            } else if buf == "eraserd" ||  buf == "erased" {
                mode = Mode::Dream;
                buf = "d".to_string();
                continue;
            } else if buf == "erasere" ||  buf == "erasee" {
                mode = Mode::Erase;
                buf = "e".to_string();
                continue;
            } else {
                break;
            }
        } 
    }
    if status {
        println!("YES");
    } else {
        println!("NO");
    }
}

pub fn call_solve10() {
    input! {
        n: usize,
        mat: [[i32; 3]; n],
    }
    let mut current = (0, 0);
    let mut result = false;
    for i in 0..n {
        result = false;
        let m = if i == 0 {mat[i][0]} else {mat[i][0] - mat[i - 1][0]};
        if (mat[i][1] - current.0).abs() + (mat[i][2] - current.1).abs() > m {
            break;
        } else if ((mat[i][1] - current.0).abs() + (mat[i][2] - current.1).abs()) % 2 != m % 2  {
            break;
        } else {
            result = true;
            current = (mat[i][1], mat[i][2]);
        }
    }
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
