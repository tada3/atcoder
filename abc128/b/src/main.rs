#[allow(unused_imports)]
use std::collections::HashSet;

//use std::collections::LinkedList;
use std::collections::VecDeque;

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

type Idx = (usize, String, u32);

fn main() {
    let n = read::<usize>();
    let mut list: VecDeque<Idx> = VecDeque::new();

    for i in 1..n+1 {
        let sp = read_vec::<String>();
        let s = sp[0].to_string();
        let p_str = &sp[1];
        let p:u32 = p_str.parse().unwrap();
        let idx = (i, s, p);

        add_idx(&mut list, idx);

    }

    for x in &list {
        println!("{}", x.0);
    }
}

fn add_idx(list : &mut VecDeque<Idx>, idx:Idx) {
    let p = find_pos(list, &idx);
    list.insert(p, idx);
}

fn find_pos(list :&VecDeque<Idx>, x: &Idx) -> usize{
    if list.len() == 0 {
        return 0;
    }
    if !cond(&list[0], x) {
        return 0;
    }

    return binary_search(list, x, 0, list.len());
}

// [ o o o x x x] となっているときは2を返す。
// We do not care the case list[mid] == x
fn binary_search(list :&VecDeque<Idx>, x: &Idx, lb : usize, ub : usize) -> usize {
    while ub - lb > 1 {
        let mid = (lb + ub) / 2;
        let mut lb1 = lb;
        let mut ub1 = ub;
        if cond(&list[mid], x) {
            lb1 = mid;
        } else {
            ub1 = mid;
        }
        return binary_search(list, x, lb1, ub1);
    }
    return ub;
}

fn cond(x : &Idx, y : &Idx) -> bool {
    if x.1 < y.1 {
        return true;
    }
    if x.1 == y.1 && x.2 > y.2 {
        return true;
    }
    return false;
}