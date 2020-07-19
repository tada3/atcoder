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
    let n = read::<usize>();
    let mut xmax: (i32, char) = (-1_000_000_000, 'N');
    let mut xmin: (i32, char) = (1_000_000_000, 'N');
    let mut ymax: (i32, char) = (-1_000_000_000, 'N');
    let mut ymin: (i32, char) = (1_000_000_000, 'N');

    let mut xmaxr: (i32, char) = (-1_000_000_000, 'R');
    let mut xminl: (i32, char) = (-1_000_000_000, 'L');
    let mut ymaxu: (i32, char) = (-1_000_000_000, 'U');
    let mut ymind: (i32, char) = (-1_000_000_000, 'D');

    let mut xyd: String;
    for i in 0..n {
        xyd = read::<String>();
        let mut sp = xyd.split_whitespace();
        let x: i32 = sp.next().unwrap().parse().unwrap();
        let y: i32 = sp.next().unwrap().parse().unwrap();
        // let d = sp.next().unwrap();

        let d = sp.next().unwrap().chars().next().unwrap();

        if x > xmax.0 {
            xmax.0 = x;
            xmax.1 = d;
        }
        if x < xmin.0 {
            xmin.0 = x;
            xmin.1 = d;
        }
        if y > ymax.0 {
            ymax.0 = y;
            ymax.1 = d;
        }
        if y < ymin.0 {
            ymin.0 = y;
            ymin.1 = d;
        }

        if d == 'R' && x > xmaxr.0 {
            xmaxr.0 = x;
        }
        if d == 'L' && x > xminl.0 {
            xminl.0 = x;
        }
        if d == 'U' && y > ymaxu.0 {
            ymaxu.0 = y;
        }
        if d == 'D' && y > ymind.0 {
            ymind.0 = y;
        }
    }





    let answer = solve(&xmax, &xmin, &ymax, &ymin);
    println!("{}", answer);
}

fn solve(xmax: &(i32, char), xmin :&(i32, char), ymax: &(i32, char), ymin :&(i32, char)) -> i32 {
    // Case 1: Zero
    if xmax == xmin || ymax == ymin {
        return 0;
    }

    


    return 0;
}
