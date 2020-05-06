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

//use lazy_static::lazy_static;

//lazy_static! {
//    static ref MOD :usize = 10usize.pow(9) + 7;
//}

use std::collections::HashMap;

static MOD: usize = 1000_000_007;

fn main() {
    let l: Vec<char> = read::<String>().chars().collect::<Vec<char>>();

    let mut pow2: HashMap<usize, usize> = HashMap::new();
    let mut pow3: HashMap<usize, usize> = HashMap::new();
    
    prepare(&l, &mut pow2, &mut pow3);
    let answer = solve(&l, &pow2, &pow3);
    println!("{}", answer);
}

fn prepare(l: &[char], m2: &mut HashMap<usize, usize>, m3: &mut HashMap<usize, usize>) {
    let n = l.len();

    let mut num_of_one = 0;
    let mut x2 = 1;
    let mut pos_of_one = n - 1;
    let mut x3 = 1;

    m2.insert(0, 1);
    m3.insert(0, 1);

    for i in (0..n).rev() {
        if l[i] == '0' {
            continue;
        }
        num_of_one += 1;
        x2 = x2 * 2 % MOD;
        m2.insert(num_of_one, x2);
        //println!("m2 {}, {}", num_of_one, x2);

        let r = pos_of_one - i;
        for j in 0..r {
            x3 = x3 * 3 % MOD;
        }
        m3.insert(n-i - 1, x3);
        pos_of_one = i;
        //println!("m3 {}, {}", r, x3);
    }
}

fn solve(l: &[char], m2: &HashMap<usize, usize>, m3: &HashMap<usize, usize>) -> usize {
    let n = l.len();

    let mut count = 0;
    let mut num_of_one = 0;
    for i in 0..n {
        if l[i] == '0' {
            continue;
        }
        let remaining = n - i - 1;
        // 0-1 or 1-0
        let c1 = m2[&num_of_one];

        //println!("XXX reamining={}", remaining);
        let c2 = m3[&remaining];
        let c3 = c1 * c2 % MOD;

        count = (count + c3) % MOD;
        num_of_one += 1;
    }
    let c1 = m2[&num_of_one];
    count = (count + c1) % MOD;
    return count;
}



fn solveX(l: &[char]) -> usize {
    let n = l.len();

    let mut count = 0;
    let mut num_of_one = 0;
    for i in 0..n {
        if l[i] == '0' {
            continue;
        }
        let remaining = n - i - 1;
        // 0-1 or 1-0
        let c1 = b_pow_a(2, num_of_one);
        let c2 = b_pow_a(3, remaining);
        let c3 = c1 * c2 % MOD;

        count = (count + c3) % MOD;
        num_of_one += 1;
    }

    let c1 = b_pow_a(2, num_of_one);
    count = (count + c1) % MOD;
    return count;
}

fn b_pow_a(a: usize, b: usize) -> usize {
    let mut x = 1;
    for _ in 0..b {
        x = x * a % MOD;
    }
    return x;
}
