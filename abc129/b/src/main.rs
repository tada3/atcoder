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
    let _n = read::<usize>();
    let w = read_vec::<i32>();

    let answer = solve(&w);
    println!("{}", answer);
}

fn solve(w: &[i32]) -> i32 {
    let sum: i32 = w.into_iter().sum();
    let mut d: i32 = -1 * sum;
    for x in w {
        let d1 = d + 2 * x;
        if d1 < 0 {
            d = d1;
            continue;
        }
        if d1 == 0 {
            return 0;
        }
        return if d1 < -d {d1} else {-d}; 
    }
    return sum;
}
