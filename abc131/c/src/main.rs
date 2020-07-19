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
    let abcd = read_vec::<u64>();
    let a = abcd[0];
    let b = abcd[1];
    let c = abcd[2];
    let d = abcd[3];
    


    let answer = solve(a, b, c, d);
    println!("{}", answer);
}

fn solve(a: u64, b:u64, c:u64, d:u64) -> u64 {
    let total = b - a + 1;
    let cc = b / c  - (a - 1) / c;
    let dd = b / d - (a - 1) / d;
    
    let l = lcm(c, d);
    let ll = b / l - (a - 1) / l;

    //println!("{}, {}, {}, {}", total, cc, dd, ll);
    
    return total - cc - dd + ll;
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut m : u64 = a;
    let mut n : u64 = b;
    if m < n {
        m = b;
        n = a;
    }

    loop {
        let r = m % n;
        if r == 0 {
            return n;
        }
        m = n;
        n = r;    
    }
}