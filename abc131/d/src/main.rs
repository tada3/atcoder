#[allow(unused_imports)]
use std::collections::HashSet;
use proconio::input;
 
type Work = (i64, i64); // A, B

fn main() {
    input! {
        n : usize,
        mut ab : [(i64, i64); n],

    }

    let answer = solve(&mut ab);
    let msg = if answer {
        "Yes"
    } else {
        "No"
    };
    println!("{}", msg);
}

fn solve(works: &mut [Work]) -> bool {
    // 1. Sort
    works.sort_by_key(|w|w.1);

    // 2. Check
    let mut total : i64 = 0;
    for w in works {
        total += w.0;
        if total > w.1 {
            return false;
        }
    }
    return true;
}
