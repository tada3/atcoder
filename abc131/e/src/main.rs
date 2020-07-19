use proconio::input;
#[allow(unused_imports)]
use std::collections::HashSet;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

type pair = (usize, usize);

// max = (n-1)(n-2)/2
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let answer = solve(n, k);
    if !answer.0 {
        println!("-1");
        return;
    }
    println!("{}", answer.1);
    for p in answer.2 {
        println!("{} {}", p.0, p.1);
    }
}

fn solve(n: usize, k: usize) -> (bool, usize, Vec<pair>) {
   
    let max = (n - 1) * (n - 2) / 2;
    //println!("XXX max={}", max);
    if k > max {
        return (false, 0, Vec::new());
    }

    // base
    // assume node n is center
    let delta = max - k;
    let mut v = Vec::<pair>::with_capacity(n-1+delta);
    for i in 1..n {
        let p = (i, n);
        v.push(p);
    }

    
    //println!("XXX delta={}", delta);
    if delta == 0 {
        return (true, n - 1, v);
    }

    // additional
    let mut count = 0;
    for i in 1..n - 1 {
        for j in i + 1..n {
            let p = (i, j);
            v.push(p);
            count += 1;
            if count == delta {
                return (true, n - 1 + delta, v);
            }
        }
    }

    return (true, n - 1 + delta, v);
}
