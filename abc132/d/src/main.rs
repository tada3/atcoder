use proconio::input;

#[allow(unused_imports)]
use std::collections::HashSet;

use std::cmp;


fn main() {
    input! {
        n: usize,
        k: usize,
    }
    solve(n, k);
}


fn prepare(x: usize, p :u64) -> Vec<Vec<u64>> {
    let mut v : Vec<Vec<u64>> = Vec::<Vec<u64>>::with_capacity(x + 1);
    let v0 : Vec<u64> = vec![1; 1];
    v.push(v0);
    for i in 1..x+1 {
        let mut vi : Vec<u64> = vec![0; i+1];
        for j in 0..i+1 {
            vi[j] = 
            if j == 0 || j == i {
                1
            } else {
                (v[i-1][j-1] + v[i-1][j]) % p
            };
        }
        v.push(vi);
        
    }
    return v;
}

fn solve(n: usize, k: usize) {
    let p : u64 = (10u32.pow(9u32) + 7) as u64;

    // 1. Calculate values of combinations
    let x = cmp::max(k-1, n-k+1);
    let c = prepare(x, p);

    // 2. Calculate possible ways
    for i in 1..k+1 {
        let b = c[k-1][i-1];
        let r = if i > n-k+1 { 
            0
        } else {
            c[n-k+1][i]
        };
        let t = b * r % p;
        println!("{}", t);
    }
}
