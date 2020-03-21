use std::cmp;

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
    let nk :Vec<usize>  = read_vec();
    let n = nk[0];
    let k = nk[1];

    let v : Vec<i32> = read_vec();

    let result = solve(n, k, &v);
    println!("{}", result);
}

fn solve(n : usize, k: usize, v:&[i32]) -> i32 {
    let s = cmp::min(n, k);
    let mut max : i32 = 0;
    for a in 0..s+1 {
        for b in 0..(s - a)+1 {
            let mut w :Vec<i32> = Vec::with_capacity(s);
            w.extend_from_slice(&v[0..a]);
            w.extend_from_slice(&v[n-b..n]);
            w.sort();
            let rest = k - (a + b);
            let limit = cmp::min(rest, a+b);
            let mut pos = 0;
            for i in 0.. limit {
                if w[i] >= 0 {
                    break;
                }
                pos += 1;
            }
            let total : i32 = w[pos..].iter().sum();
            //println!("{},{},{} -> {}", a, b, pos, total);
            if total > max {
                max = total;
            }
        }
    }
    return max;
}
