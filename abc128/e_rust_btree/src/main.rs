use std::collections::BTreeMap;
use std::collections::HashMap;

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

type Work = (i32, i32, i32); // S', T', X

fn main() {
    let nq :Vec<usize>  = read_vec();
    let n = nq[0];
    let q = nq[1];


    let mut works : Vec<Work> = Vec::with_capacity(n);
    for _i in 0..n {
        let stx :Vec<i32> = read_vec();
        let s = stx[0] - stx[2];
        let t = stx[1] - stx[2];
        let w : Work = (s, t, stx[2]);
        works.push(w);
    }

    let mut starts = BTreeMap::new();
    for i in 0..q {
        let d : i32 = read();
        starts.insert(d, i);
    }

    let result = solve(q, &mut works, &mut starts);
    //result.iter().for_each(|x|println!("{}", x));
    for stop in &result {
        println!("{}", stop);
    }
}

fn solve(q: usize, works : &mut [Work], starts: &mut BTreeMap<i32, usize>) -> Vec<i32> {
    let mut stops : Vec<i32> = vec![-1; q];

    // 1. Sort (NlogN)
    works.sort_by_key(|x| x.2);

    // 2. Find intercepted person (QlogQ)
    let mut intercepted :Vec<i32> = Vec::with_capacity(q);
    for w in works {
        if starts.len() == 0 {
            break;
        }

        for d in starts.range(w.0..) {
            if *d.0 >= w.1 {
                break;
            }
            stops[*d.1] = w.2;
            intercepted.push(*d.0)
        }

        for i in &intercepted {
            starts.remove(i);
        }
    }

    return stops;
}

fn solveXXX(q: usize, works : &mut [Work], starts: &[i32]) -> Vec<i32> {
    let mut stops : Vec<i32> = vec![-1; q];
    for w in works {
        let m = get_intercepted(q, starts, w);
        //println!("{:?} -> {:?}", w, m);
        if m == (q, q) {
            break;
        }
        for i in m.0..m.1 {
            if stops[i] < 0 || w.2 < stops[i] {
                stops[i] = w.2;
            }
        }
    }
    return stops;
}

fn get_intercepted(q: usize, starts: &[i32], w: &Work) -> (usize, usize) {
    let m = binary_search(q, starts, w);
    if m == q {
        return (q, q);
    }

    // check neighbors
    let mut from = m;
    let mut to = m + 1;
    for i in (0..m).rev() {
        if starts[i] < w.0 {
            from = i + 1;
            break;
        }
    }

    for i in m+1..q {
        if w.1 <= starts[i] {
            to = i;
            break;
        }
    }

    return (from, to);
}

fn binary_search(q: usize, starts: &[i32], w: &Work) -> usize {
    let mut lo = 0;
    let mut hi = q - 1;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if starts[mid] < w.0 {
            if mid == q - 1 {
                break;
            }
            lo = mid + 1;
        }else if w.1 <= starts[mid] {
            if mid == 0 {
                break;
            }
            hi = mid - 1;
        } else {
            return mid;
        }
    }
    return q;
}
