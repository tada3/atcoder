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
    let whxy = read_vec::<i32>();
    let w = whxy[0];
    let h = whxy[1];
    let x= whxy[2];
    let y = whxy[3];

    let answer = solve(w, h, x, y);
    println!("{} {}", answer.0, answer.1);
}

fn solve(w: i32, h: i32, x:i32, y:i32) -> (f64, i32) {
    let half = (w as f64) * (h as f64) / 2.0;

    let is_center = 
    if x == w / 2 && w % 2 == 0 && y == h / 2 && h % 2 == 0 {
        1
    } else {
        0
    };

    return (half, is_center);
}
