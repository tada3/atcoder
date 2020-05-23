#[allow(unused_imports)]
use std::collections::HashMap;

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

const MOD : i32 = 1_000_000_007;

  

fn main() {
    let nm = read_vec::<usize>();
    let n = nm[0];
    let m = nm[1];
    let s = read_vec::<u32>();
    let t = read_vec::<u32>();

    let answer = solve(n, m, &s, &t);
    println!("{}", answer);
}

fn solve(n: usize, m: usize, s: &[u32], t: &[u32]) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; m+1]; n+1];
    let mut sum: Vec<Vec<i32>> = vec![vec![0; m+1]; n+1];

    dp[0][0] = 0;
    sum[0][0] = 1;

    for i in 1..n + 1 {
        dp[i][0] = 0;
        sum[i][0] = 1;
    }

    for j in 1..m + 1 {
        dp[0][j] = 0;
        sum[0][j] = 1;
    }

    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = sum[i - 1][j - 1];
            }
            sum[i][j] = sum[i][j-1] + sum[i-1][j] - sum[i-1][j-1] + dp[i][j];
            sum[i][j] %= MOD;
        }
    }

    return sum[n][m];
}
