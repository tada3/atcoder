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
    let nk = read_vec::<u64>();
    let a = read_vec::<u32>();
    let n = nk[0] as usize;
    let k = nk[1];

    let answer = solve(n, k, &a);
    println!("{}", answer);
}

fn solve(n: usize, k: u64, a: &[u32]) -> u64 {
    let mut count: u64 = 0;
    let mut sum: u64 = 0;
    let mut t = 0;
    for i in 0..n {
        //println!("{}, {}, {}, {}", i, t, sum, count);
        if sum < k {
            //println!("try!");
            for j in t..n {
                //println!("j : {}", j);
                sum += a[j] as u64;
                if sum >= k {
                    t = j + 1;
                    break;
                }
            }
            if sum < k {
                //println!("Reached end of array but still too small!");
                break;
            }
        }
        count += (n - t + 1) as u64;
        sum -= a[i] as u64;
    }

    return count;
}
