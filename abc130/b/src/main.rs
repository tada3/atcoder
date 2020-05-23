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
    let nx = read_vec::<usize>();
    let n = nx[0];
    let x = nx[1];
    let mut l = read_vec::<usize>();
    l.push(x);
    let answer = solve(n, x, &l);

    println!("{}", answer);
}

fn solve(n: usize, x:usize, l:&[usize]) -> usize {
    let mut pos = 0;
    let mut count = 0;
    for &a in l {
        if pos > x {
            return count;
        }
        pos += a;
        count += 1;
    }
    return count;
}
