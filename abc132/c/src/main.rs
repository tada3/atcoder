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
        mut d: [i32; n] 
    }
    let answer = solve(n, &mut d);
    println!("{}", answer);
}

fn solve(n: usize, d: &mut [i32]) -> i32 {
    // 1. sort
    d.sort();

    // 2. threshold should middle of d
    //     possible ways = diff between <left of middle> and <right of middle>
    let mid = n / 2;
    return d[mid] - d[mid-1];

}
