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

    let mut shogai_yoko : Vec<Vec<usize>> = Vec::with_capacity(h);
    let mut shogai_tate : Vec<Vec<usize>> = Vec::with_capacity(w);
    for _i in 0..w {
        let v : Vec<usize> = Vec::with_capacity(h);
        shogai_tate.push(v)
    }
    
    for i in 0..h {
        let mut v : Vec<usize> = Vec::with_capacity(w);
        let s = read::<String>();
        for (j, c) in s.chars().enumerate() {
            if c == '#' {
                v.push(j);
                shogai_tate[j].push(i)
            }
        }
       
        if v.is_empty() || v[v.len()-1] < w - 1 {
            v.push(w);
        }
        shogai_yoko.push(v);
    }
    for j in 0..w {
        let v = &mut shogai_tate[j];
        if v.is_empty() || v[v.len() - 1] < h - 1 {
            v.push(h)
        }
    }


    let answer = solve(h, w, &shogai_yoko, &shogai_tate);
    println!("{}", answer);
}

fn solve(h: usize, w: usize, shogai_yoko: &[Vec<usize>], shogai_tate: &[Vec<usize>]) -> usize {
     // (yoko, tate)
     let mut dp : Vec<Vec<usize>> = Vec::with_capacity(h);
     for _i in 0..h {
        let v : Vec<usize> = vec![0; w];
        dp.push(v);
     }

     // yoko
     for i in 0..h {
        let mut from = 0;
        for x in &shogai_yoko[i] {
            let d = x - from;
            if d > 1 {
                for j in from..*x {
                    dp[i][j] = d - 1;
                }
            }
            from = x + 1;
        }
    }

    // tate (and total)
    let mut max = 0;
    let limit = (h-1) + (w-1);
    for j in 0..w {
        let mut from = 0;
        for x in &shogai_tate[j] {
            let d = x - from;
            if d > 0 {
                for i in from..*x {
                    let val = dp[i][j] + (d - 1);
                    if val > max {
                        max = val;
                        if max == limit {
                            return max + 1;
                        }
                    }
                }
            }
            from = x + 1;
        }
    }
    return max + 1
}


fn solve2(h: usize, w: usize, shogai_yoko: &[Vec<usize>], shogai_tate: &[Vec<usize>]) -> usize {
    // (yoko, tate)
    let mut dp : Vec<Vec<usize>> = Vec::with_capacity(h);
    for _i in 0..h {
       let v : Vec<usize> = vec![0; w];
       dp.push(v);
    }

    // yoko
    for i in 0..h {
       let mut from = 0;
       for x in &shogai_yoko[i] {
           let d = x - from;
           if d > 0 {
               for j in from..*x {
                   dp[i][j] = d - 1;
               }
           }
           from = x + 1;
       }
   }

   // tate (and total)
   let mut max = 0;
   for j in 0..w {
       let mut from = 0;
       for x in &shogai_tate[j] {
           let d = x - from;
           if d > 0 {
               for i in from..*x {
                   let val = dp[i][j] + (d - 1);
                   if val > max {
                       max = val
                   }
               }
           }
           from = x + 1;
       }
   }
   return max + 1
}



fn solve1(h: usize, w: usize, shogai_yoko: &[Vec<usize>], shogai_tate: &[Vec<usize>]) -> usize {
    // (yoko, tate)
    let mut dp : Vec<Vec<(usize, usize)>> = Vec::with_capacity(h);
    for _i in 0..h {
       let v : Vec<(usize, usize)> = vec![(0, 0); w];
       dp.push(v);
    }

    // yoko
    for i in 0..h {
       let mut from = 0;
       for x in &shogai_yoko[i] {
           let d = x - from;
           if d > 0 {
               for j in from..*x {
                   dp[i][j].0 = d - 1;
               }
           }
           from = x + 1;
       }
   }

   // tate
   for j in 0..w {
       let mut from = 0;
       for x in &shogai_tate[j] {
           let d = x - from;
           if d > 0 {
               for i in from..*x {
                   dp[i][j].1 = d - 1;
               }
           }
           from = x + 1;
       }
   }

   // total
   let mut max = 0;
   for i in 0..h {
       for j in 0..w {
           let val = dp[i][j].0 + dp[i][j].1 + 1;
           if val > max {
               max = val;
           }
       }
   }

   return max
}
