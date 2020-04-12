#[allow(unused_imports)]
use std::collections::HashSet;

#[allow(dead_code)]
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
    let hw = read_vec::<usize>();
    let h = hw[0];
    let w = hw[1];

    let mut masu : Vec<Vec<char>> = Vec::with_capacity(h);
    for _i in 0..h {
        let s : Vec<char> = read::<String>().chars().collect();
        masu.push(s);
    }

    let answer = solve(h, w, &masu);
    println!("{}", answer);
}

fn solve(h: usize, w: usize, masu: &[Vec<char>]) -> usize {
    let mut dp : Vec<Vec<(usize, usize)>> = vec![vec![(0, 0);w];2];
    let mut max = 0;
    
    for i in 0..h {
        let curr = i % 2;
        for j in 0..w {
            if masu[i][j] == '#' {
                dp[curr][j] = (0, 0);
                continue;
            }

            // yoko
            if j == 0 || masu[i][j-1] == '#' {
                let mut j1 = j + 1;
                while j1 < w && masu[i][j1] != '#' {
                    j1 += 1;
                }
                dp[curr][j].0 = j1 - j - 1; 
            } else {
                dp[curr][j].0 = dp[curr][j-1].0;
            }

            // tate
            if i == 0 || masu[i-1][j] == '#' {
                let mut i1 = i + 1;
                while i1 < h && masu[i1][j] != '#' {
                    i1 += 1;
                }
                dp[curr][j].1 = i1 - i - 1;
            } else {
                dp[curr][j].1 = dp[1-curr][j].1;
            }

            let val = dp[curr][j].0 + dp[curr][j].1;
            if val > max {
                max = val;
            }
        }
    }
    return max + 1;
}
