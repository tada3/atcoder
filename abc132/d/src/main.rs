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

fn prepare(m: usize, k: usize, p :u64) -> Vec<Vec<u64>> {
    let mut v : Vec<Vec<u64>> = Vec::<Vec<u64>>::with_capacity(m + 1);
    let v0 : Vec<u64> = vec![1; 1];
    v.push(v0);
    for i in 1..m+1 {
        let mut vi : Vec<u64>;
        if i <= k {
            vi = vec![0; i+1];
            vi[0] = 1;
            vi[i] = 1;
            for j in 1..i/2+1 {
                vi[j] = (v[i-1][j-1] + v[i-1][j]) % p;
                vi[i-j] = vi[j];
            }
        }
        else {
            vi = vec![0; k+1];
            vi[0] = 1;
            if i < 2 * k {
                for j in 1..i/2+1 {
                    vi[j] = (v[i-1][j-1] + v[i-1][j]) % p;
                    let jx = i - j;
                    if jx <= k {
                        vi[jx] = vi[j];
                    }
                }
            } else {
                for j in 1..k+1 {
                    vi[j] = (v[i-1][j-1] + v[i-1][j]) % p;
                }
            } 
        }
        v.push(vi);
    }
    return v;
}

fn solve(n: usize, k: usize) {
    let p : u64 = (10u32.pow(9u32) + 7) as u64;

    // 1. Calculate values of combinations
    let m = cmp::max(k-1, n-k+1);
    let c = prepare(m, k, p);

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
