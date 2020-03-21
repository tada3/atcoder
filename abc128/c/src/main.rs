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
    let nm = read_vec::<usize>();
    let n = nm[0];
    let m = nm[1];

    let mut bitsv : Vec<u32> = Vec::with_capacity(m);
    for _i in 0..m {
        let kss = read_vec::<usize>();
        let k = kss[0];
        let mut bits : u32 = 0;
        for j in 1..k+1 {
            let s = kss[j] as u32;
            bits = bits | (1 << (s-1));
        }
        bitsv.push(bits);
    }

    let ps = read_vec::<u32>();

    let num = solve_bits(m, n, &bitsv, &ps);

    println!("{}", num);
}

fn solve_bits(m: usize, n: usize, bitsv: &[u32], ps: &[u32]) -> u32 {
    let mut total : u32 = 0;
    for x in 0..1<<n {
        let mut tsuku : bool = true;
        for i in 0..m {
            let turned_ons = x & bitsv[i];
            let c_one = count_1(turned_ons);
            if c_one % 2 != ps[i] {
                tsuku = false;
                break;
            }
        }
        if tsuku {
            total += 1;
        }

    }
    return total;
}

fn count_1(x : u32) -> u32 {
    let mut xx = x;
    let mut c : u32 = 0;
    while  xx > 0 {
        if xx & 1 > 0 {
            c += 1;
        }
        xx = xx >> 1;
    }
    return c;
}

fn mainGJ() {
    let nm = read_vec::<usize>();
    let n = nm[0];
    let m = nm[1];

    let mut a : Vec<Vec<i32>> = Vec::with_capacity(m);
    for _i in 0..m {
        let mut row:Vec<i32> = vec![0;n];

        let kss = read_vec::<usize>();
        let k = kss[0];
        for j in 1..k+1 {
            let s = kss[j];
            row[s-1] = 1;
        }
        a.push(row);
    }

    let ps = read_vec::<i32>();
    for i in 0..m {
        a[i].push(ps[i]);
    }

    // println!("a: {:?}", a);


    let num = solve(m, n, &mut a);

    println!("{}", num);

}

fn solve(m :usize, n: usize, a: &mut [Vec<i32>]) -> usize {
    let rank = gauss_jordan_mod2(m, n, a);

    //println!("rank: {}", rank);

    //println!("a: {:?}", a);

    if rank < m {
        for i in rank..m {
            if a[i][n] != 0 {
                return 0;
            }
        }
    }

    let mut num = 1;
    for _i in rank..n {
        num *= 2;
    }
    return num;
}


// Get rank for m rows n+1 columns augmented matrix
fn gauss_jordan_mod2(m :usize, n: usize, a: &mut [Vec<i32>]) -> usize {
    let mut rank : usize = 0;

    for j in 0..n {
        //println!("LOOP {}: {:?}", j, a);
        let mut pivot : usize = m;
        // 1. Get pivot
        for i in rank..m {
            if a[i][j] != 0 {
                pivot = i;
                break;
            }
        }

        if pivot == m {
            continue;
        }

        // 2. Swap
        a.swap(rank, pivot);

        // 3. Set a[i][j] (i != rank) to zero
        for i in 0..m {
            if i == rank {
                continue;
            }
            if a[i][j] == 0 {
                continue;
            }
            for j1 in 0..n+1 {
                a[i][j1] -= a[rank][j1];
                if a[i][j1] < 0 {
                    a[i][j1] += 2;
                }
            }
        }

        rank += 1;
    }

    return rank;
}
