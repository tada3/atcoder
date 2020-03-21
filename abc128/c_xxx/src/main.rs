
// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
 
// 標準入力から一行を読み取り、空白文字で分割し、各要素を指定の型に変換する関数
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
 
//use std::cmp::*;

use rand::Rng;

fn get_nm() -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut nm : Vec<usize> = Vec::with_capacity(2);
    let n = rng.gen_range(1, 11);
    let m = rng.gen_range(1, 11);
    nm.push(n);
    nm.push(m);
    return nm;
}

fn get_ps(m : usize) -> Vec<u32> {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します

    let mut ps: Vec<u32> = vec![0;m];
    for i in 0..m {
        let r: bool = rng.gen();
        if r {
            ps[i] = 1;
        }
    }
    return ps;
}


fn get_kmv(m: usize, n: usize) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します

    let mut kmv: Vec<Vec<u32>> = Vec::with_capacity(m);
    for i in 0..m {
        let mut km : Vec<u32> = Vec::with_capacity(n+1);
        let mut count : u32 = 0;
        km.push(0);
        for j in 0..n {
            let r: bool = rng.gen();
            if r {
                km.push((j + 1) as u32);
                count += 1;
            }
        }
        km[0] = count;
        kmv.push(km);
    }
    return kmv;
}


fn main() {
   //let nm: Vec<usize> = read_vec();
    let nm = get_nm(); 
   let n = nm[0];
    let m = nm[1];
    // 対象となるスイッチを表すbit列
    // ex. スイッチが5個で、i個目の電球に紐づいたスイッチが1,2,5
    // => 10011

    /**
    let mut kmv : Vec<Vec<u32>> = Vec::with_capacity(m);
 
    for _i in 0..m
    {
        let km: Vec<u32> = read_vec();
        kmv.push(km);
    }
    **/
    let kmv = get_kmv(m, n);
 
    //let p: Vec<u32> = read_vec();
    let ps = get_ps(m);

    
    let result = solveXXX(n, m, &kmv, &ps);
    let mine = solveMine(n, m, &kmv, &ps);

 
    println!("{}, {}", result, mine);
    if result != mine {
        println!("{} {}", n, m);
        for i in 0..m {
            println!("{:?}", kmv[i]);
        }
        println!("{:?}", ps);
    } 
}
fn mainXXX() {
    let nm: Vec<usize> = read_vec();
    let n = nm[0];
    let m = nm[1];
    // 対象となるスイッチを表すbit列
    // ex. スイッチが5個で、i個目の電球に紐づいたスイッチが1,2,5
    // => 10011
    let mut bits: Vec<u64> = vec![0; m];
 
    for i in 0..m
    {
        let km: Vec<u32> = read_vec();
        for j in 0..km[0] as usize
        {
            bits[i] += 1 << (km[1 + j] - 1);
        }
    }
 
    let p: Vec<u32> = read_vec();




    let mut result: u64 = 0;
 
    // n個のスイッチに関してbit全探索
    'outer: for bp in 0..1 << n
    {
        // 各電球が点灯するか調べる, 点灯しないパターンが存在する場合はcontinue
        for i in 0..m
        {
            // AND
            let and = bits[i] & bp;
            if and.count_ones() % 2 != p[i] { continue 'outer; }
        }
 
        result += 1;
    }
 
    println!("{}", result);
}

fn solveXXX(n: usize, m: usize, kmv: &[Vec<u32>], p: &[u32]) -> u64 {
    let mut bits: Vec<u64> = vec![0; m];

    for i in 0..m
    {
        // let km: Vec<u32> = read_vec();
        for j in 0..kmv[i][0] as usize
        {
            bits[i] += 1 << (kmv[i][1 + j] - 1);
        }
    }

    let mut result: u64 = 0;
 
    // n個のスイッチに関してbit全探索
    'outer: for bp in 0..1 << n
    {
        // 各電球が点灯するか調べる, 点灯しないパターンが存在する場合はcontinue
        for i in 0..m
        {
            // AND
            let and = bits[i] & bp;
            if and.count_ones() % 2 != p[i] { continue 'outer; }
        }
 
        result += 1;
    }
    return result;
}

fn solveMine(n: usize, m: usize, kmv: &[Vec<u32>], ps: &[u32]) -> u64 {


    let mut a : Vec<Vec<i32>> = Vec::with_capacity(m);
    for i in 0..m {
        let mut row:Vec<i32> = vec![0;n];

        //let kss = read_vec::<usize>();
        let kss = &kmv[i];
        let k = kss[0];
        for j in 1..k+1  {

            let s = kss[j as usize];
            row[(s-1) as usize] = 1;
        }
        a.push(row);
    }

    //let ps = read_vec::<i32>();
    for i in 0..m {
        a[i].push(ps[i] as i32);
    }

    // println!("a: {:?}", a);


    let num = solve(m, n, &mut a);
    return num as u64;
}




fn solve(m :usize, n: usize, a: &mut [Vec<i32>]) -> usize {
    let rank = gauss_jordan_mod2(m, n, a);

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
