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

    let mut shogai_tab : Vec<HashSet<usize>> = Vec::with_capacity(h);
    
    for i in 0..h {
        let mut set : HashSet<usize> = HashSet::new();
        let s = read::<String>();
        for (i, c) in s.chars().enumerate() {
            if c == '#' {
                set.insert(i);
            }
        }
        shogai_tab.push(set);
    }


    let answer = solve(h, w, &shogai_tab);
    println!("{}", answer);
}

fn solve(h: usize, w: usize, shogai: &[HashSet<usize>]) -> usize {
    // yoko, tate
    let mut dp : Vec<Vec<(usize, usize)>> = Vec::with_capacity(h);
    for _i in 0..h {
        let v : Vec<(usize, usize)> = vec![(0, 0); w];
        dp.push(v);
    }

    let mut max : usize = 0;
    for i in 0..h {
        let shogaiC = &shogai[i];
        let shogaiP = &shogai[i-1];
        for j in 0..w {
           // if shogai[i].contains(&j) {
            if shogaiC.contains(&j) {
                continue;
            }

            // yoko
            if is_left_shogai_or_kabe2(i, j, shogaiC) {
                // right
                dp[i][j].0 = check_right2(w, i, j, shogaiC);
            } else {
                dp[i][j].0 = dp[i][j-1].0;
            }

            // tate
            if is_up_shogai_or_kabe2(i, j, shogaiP) {
                // down
                dp[i][j].1 = check_down(h, i, j, shogai);
            } else {
                dp[i][j].1 = dp[i-1][j].1;
            }

            //println!("{}, {} -> {}, {}", i, j, dp[i][j].0, dp[i][j].1);
            let val = dp[i][j].0 + dp[i][j].1 + 1;
            if val > max {
                max = val;
            }
        }
    }

    return max;
}


fn check_right(w: usize, i : usize, j: usize, shogai: &[HashSet<usize>]) -> usize {
    let mut p = j + 1;
    while p < w && !shogai[i].contains(&p) {
        p += 1;
    }
    return p - j - 1;
}

fn check_right2(w: usize, i : usize, j: usize, shogai: &HashSet<usize>) -> usize {
    let mut p = j + 1;
    while p < w && !shogai.contains(&p) {
        p += 1;
    }
    return p - j - 1;
}

fn check_down(h: usize, i : usize, j: usize, shogai: &[HashSet<usize>]) -> usize {
    let mut p = i + 1;
    while p<h && !shogai[p].contains(&j) {
        p += 1;
    }
    return p - i - 1;
}

fn is_left_shogai_or_kabe(i: usize, j: usize, shogai: &[HashSet<usize>]) -> bool {
    if j == 0 {
        return true;
    }
    return shogai[i].contains(&(j-1));
}


fn is_left_shogai_or_kabe2(i: usize, j: usize, shogai: &HashSet<usize>) -> bool {
    if j == 0 {
        return true;
    }
    return shogai.contains(&(j-1));
}

fn is_up_shogai_or_kabe(i: usize, j: usize, shogai: &[HashSet<usize>]) -> bool {
    if i == 0 {
        return true;
    }
    return shogai[i-1].contains(&j);
}

fn is_up_shogai_or_kabe2(i: usize, j: usize, shogai: &HashSet<usize>) -> bool {
    if i == 0 {
        return true;
    }
    return shogai.contains(&j);
}