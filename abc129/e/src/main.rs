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

//use lazy_static::lazy_static;

//lazy_static! {
//    static ref MOD :usize = 10usize.pow(9) + 7;
//}

use std::collections::HashMap;

static MOD: usize = 1000_000_007;

fn main() {
    let l: Vec<char> = read::<String>().chars().collect::<Vec<char>>();
    let n = l.len();

    let mut pow3 : Vec<usize> = vec![0; n];
    prepare(n, &l, &mut pow3);
    let answer = solve(n, &l, &pow3);
    println!("{}", answer);
}


fn prepare(n: usize, l: &[char], m3: &mut[usize]) {
    let mut x3 = 1;
    for i in 0..n { 
        m3[i] = x3;
        x3 = x3 * 3 % MOD;
    }
}


fn solve(n: usize, l: &[char], m3: &[usize]) -> usize {
    let mut count = 0;
    let mut c1 = 1;
    for i in 0..n {
        if l[i] == '0' {
            continue;
        }
        let remaining = n - i - 1; 
        let c2 = m3[remaining];
        let c3 = c1 * c2 % MOD;
        count = (count + c3) % MOD;
        
        c1 = c1 * 2 % MOD;
    }
    count = (count + c1) % MOD;
    return count;
}
