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

fn main() {
    let xa = read_vec::<i32>();
    let x = xa[0];
    let a = xa[1];
    
    let answer = solve(x, a);
    println!("{}", answer);
}

fn solve(x: i32, a: i32) -> i32 {
    if x < a {
        return 0;
    }
    
    return 10;
}
