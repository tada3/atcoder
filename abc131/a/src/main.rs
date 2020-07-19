#[allow(unused_imports)]
use std::collections::HashSet;
use std::str::Chars;

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
    let s = read::<String>();
    let s1 = s.chars();
   
    let answer = solve(s1);
    if answer {
        println!("Bad");
    } else {
        println!("Good")
    }
}

fn solve(s: Chars) -> bool {
    let mut prev  = 'X';
    for c in s {
        if c == prev {
            return true;
        }
        prev = c
    }
    
    return false;
}
