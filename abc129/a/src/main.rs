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
    let abc = read_vec::<u32>();
    

    let answer = solve(&abc);
    println!("{}", answer);
}

fn solve(abc: &[u32]) -> u32 {
    if abc[0] <= abc[1] {
        if abc[1] <= abc[2] {
            return abc[0] + abc[1];
        } 
        return abc[0] + abc[2];
    } else {
        if abc[0] <= abc[2] {
            return abc[0] + abc[1];
        }
        return abc[1] + abc[2];
    }
}
