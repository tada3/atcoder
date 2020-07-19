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
    let nl = read_vec::<i32>();
    let n = nl[0];
    let l = nl[1];

    let answer = solve(n, l);
    println!("{}", answer);
}

fn solve(n: i32, l : i32) -> i32 {
    let total = (l + l + n - 1) * n / 2;
    if l > 0 {
        return total - l;
    }
    if l + n - 1 >= 0 {
        return total;
    }
    return total - (l + n - 1);
}
