use std::collections::HashMap;

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

const MOD: u64 = 1_000_000_007;

fn main() {
    let nm = read_vec::<usize>();
    let n = nm[0];
    let m = nm[1];

    let mut a : Vec<usize> = Vec::with_capacity(m + 1);
    for i in 0..m {
        a.push(read::<usize>());
    }
    a.push(n + 1);


    let answer = solve(n, m, &a);
    println!("{}", answer);
}

fn solve(n: usize,m: usize, a: &[usize]) -> u64 {

    let mut amap : HashMap<usize, u32> = HashMap::new();

    let mut from = 0;
    let mut max = 0;
    for k in a {
        if *k == from {
            return 0;
        }
        let d = k - 1 - from;
        if d > 0 {
            let count = amap.entry(d).or_insert(0);
            *count += 1;

            if d > max {
                max = d;
            }
        }
        from = k + 1;
    }

    let mut f0 = 0;
    let mut f1 = 1;
    let mut s : u64 = 1;
    for i in 1..max + 1 {
        let f = (f0 + f1) % MOD;
        
        if let Some(count) = amap.get(&i) {
            for p in 0..*count {
                s = (s * f) % MOD;
            }
        }

        f0 = f1;
        f1 = f % MOD;
    }


    return s;
}
