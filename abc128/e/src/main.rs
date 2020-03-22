use std::fmt;
use std::iter::IntoIterator;
use std::iter::Iterator;

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
    let nq: Vec<usize> = read_vec();
    let n = nq[0];
    let q = nq[1];

    let mut works: Vec<Work> = Vec::with_capacity(n);
    for _i in 0..n {
        let stx: Vec<i32> = read_vec();
        let s = stx[0] - stx[2];
        let t = stx[1] - stx[2];
        let w: Work = (s, t, stx[2]);
        works.push(w);
    }

    //let mut starts = BTreeMap::new();
    // 11 is the same as the order of rust BTreeMap
    let mut starts = BTree::<i32, usize>::new_with(11);
    for i in 0..q {
        let d: i32 = read();
        starts.insert(d, i);
    }

    let result = solve(q, &mut works, &mut starts);
    //result.iter().for_each(|x|println!("{}", x));
    for stop in &result {
        println!("{}", stop);
    }
}

fn solve(q: usize, works: &mut [Work], starts: &mut BTree<i32, usize>) -> Vec<i32> {
    let mut stops: Vec<i32> = vec![-1; q];

    // 1. Sort (NlogN)
    works.sort_by_key(|x| x.2);

    // 2. Find intercepted person (QlogQ)
    let mut intercepted: Vec<i32> = Vec::with_capacity(q);
    for w in works {
        if starts.is_empty() {
            break;
        }

        {
            let it = &mut starts.iter();
            it.move_to(&w.0);
            for d in it {
                if *d.0 >= w.1 {
                    break;
                }
                stops[*d.1] = w.2;
                intercepted.push(*d.0)
            }
        }

        for i in &intercepted {
            starts.remove(i);
        }
        intercepted.clear();
    }

    return stops;
}



struct BTree<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    m: usize,
    root: Node<K, V>,
}

// Separate keys and values.
// In the search, I need to acess only keys.
// Having keys as a dedicated data structure makes
// the search faster.
// More work is required in the insert, but it does not matter
// in a usual case.
struct Node<K, V>
where
    K: Ord,
{
    ks: Vec<K>,
    vs: Vec<V>,
    ns: Vec<Node<K, V>>,
}

struct NodeIter<'a, K: 'a, V: 'a>
where
    K: Ord,
    K: fmt::Display,
{
    node: &'a Node<K, V>,
    pos: usize,
    go_child: bool,
}

struct BTreeIterator<'a, K: 'a, V: 'a>
where
    K: Ord,
    K: fmt::Display,
{
    stack: Vec<NodeIter<'a, K, V>>,
    curr: NodeIter<'a, K, V>,
}

impl<K, V> BTree<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    fn new() -> BTree<K, V> {
        BTree::<K, V>::new_with(3)
    }

    fn new_with(order: usize) -> BTree<K, V> {
        BTree::<K, V> {
            m: order,
            root: Node::<K, V>::new(order),
        }
    }

    fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    fn iter_from(&self, x: &K) -> BTreeIterator<K, V> {
        let mut it = self.iter();

        loop {
            let pos = it.curr.node.find_pos(x);
            if pos.1 {
                it.curr.pos = pos.0;
                it.curr.go_child = false;
            }

            if it.curr.node.is_leaf() {}
        }
    }

    fn iter(&self) -> BTreeIterator<K, V> {
        BTreeIterator {
            stack: Vec::new(),
            curr: NodeIter {
                node: &self.root,
                pos: 0,
                go_child: true,
            },
        }
    }

    fn insert(&mut self, x: K, v: V) {
        let need_split = self.root.insert(x, v, self.m);
        if need_split {
            let mut tmp = Node::new(self.m);
            std::mem::swap(&mut tmp, &mut self.root);

            let split = tmp.split(self.m);

            self.root.ks.push(split.0);
            self.root.vs.push(split.1);
            self.root.ns.push(tmp);
            self.root.ns.push(split.2);
        }
    }

    fn remove(&mut self, x: &K) -> Option<V> {
        let result = self.root.remove(x, self.m);
        if self.root.is_empty() && !self.root.ns.is_empty() {
            let newRoot = self.root.ns.pop().unwrap();
            self.root = newRoot;
        }
        return result.0;
    }

    fn find(&self, x: &K) -> Option<&V> {
        return self.root.find(x);
    }

    fn print(&self) {
        println!("{}", self.root);

        let mut next = Vec::<&Node<K, V>>::with_capacity(10);
        for n in &self.root.ns {
            next.push(n);
        }

        loop {
            if next.len() == 0 {
                break;
            }

            for n in &next {
                print!("{} ", n);
            }
            println!();

            let mut tmp = Vec::<&Node<K, V>>::with_capacity(10);
            for n in &next {
                for c in &n.ns {
                    tmp.push(c);
                }
            }

            std::mem::swap(&mut tmp, &mut next);
        }

        println!();
    }
}

impl<'a, K, V> IntoIterator for &'a BTree<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    type Item = (&'a K, &'a V);
    type IntoIter = BTreeIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

const CHECK_RIGHT_MOST_THRESHOLD : usize = 7;
const USE_LINEAR_THRESHOLD : usize = 7;

impl<K, V> Node<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    fn new(m: usize) -> Node<K, V> {
        Node::<K, V> {
            ks: Vec::<K>::with_capacity(m - 1),
            vs: Vec::<V>::with_capacity(m - 1),
            ns: Vec::<Node<K, V>>::with_capacity(m),
        }
    }

    fn is_leaf(&self) -> bool {
        self.ns.is_empty()
    }

    fn is_empty(&self) -> bool {
        self.ks.is_empty()
    }

    fn insert(&mut self, x: K, v: V, m: usize) -> bool {
        let pos = self.find_pos(&x);
        if self.is_leaf() {
            if pos.1 {
                // overwrite
                self.ks[pos.0] = x;
                self.vs[pos.0] = v;
                return false;
            }
            // insert
            self.ks.insert(pos.0, x);
            self.vs.insert(pos.0, v);
            return self.ks.len() == m;
        }

        let need_split = self.ns[pos.0].insert(x, v, m);
        if need_split {
            self.split_child(pos.0, m);
            return self.ks.len() == m;
        }

        return false;
    }

    fn remove(&mut self, x: &K, m: usize) -> (Option<V>, bool) {
        let pos = self.find_pos(&x);
        if self.is_leaf() {
            // 1. Leaf
            if !pos.1 {
                // Not found
                return (None, false);
            }
            self.ks.remove(pos.0);
            let removed = self.vs.remove(pos.0);
            return (Some(removed), self.ks.len() == 0);
        }

        // 2. Non-leaf
        if pos.1 {
            // 2.1. Remove from this node
            let rm = self.ns[pos.0].remove_right_most(m);

            self.ks.remove(pos.0);
            let v = self.vs.remove(pos.0);

            self.ks.insert(pos.0, rm.0);
            self.vs.insert(pos.0, rm.1);

            if rm.2 {
                self.borrow_or_merge_from_right(pos.0, m);
            }
            return (Some(v), self.ks.len() < m / 2);
        }

        // 2.2. Remove from child
        let result = self.ns[pos.0].remove(x, m);
        if result.1 {
            if pos.0 == self.ks.len() {
                self.borrow_or_merge_from_left(pos.0, m);
            } else {
                self.borrow_or_merge_from_right(pos.0, m);
            }
            return (result.0, self.ks.len() < m / 2);
        }
        return result;
    }

    fn find(&self, x: &K) -> Option<&V> {
        let pos = self.find_pos(x);
        if pos.1 {
            return Some(&self.vs[pos.0]);
        }

        if self.ns.len() == 0 {
            return None;
        }

        return self.ns[pos.0].find(x);
    }

    fn split_child(&mut self, pos: usize, m: usize) {
        let result = self.ns[pos].split(m);
        self.ks.insert(pos, result.0);
        self.vs.insert(pos, result.1);
        self.ns.insert(pos + 1, result.2);
    }

    // Split self into two nodes (self and right).
    fn split(&mut self, m: usize) -> (K, V, Node<K, V>) {
        let mut right = Node::new(m);

        let mid = m / 2;

        right.ks = self.ks.split_off(mid + 1);
        right.vs = self.vs.split_off(mid + 1);

        let midE = self.ks.pop().unwrap();
        let midV = self.vs.pop().unwrap();

        if self.ns.len() > 0 {
            right.ns = self.ns.split_off(mid + 1);
        }
        return (midE, midV, right);
    }

    // For small m (around 11), linear search is better than 
    // binary search. But the plain linear search is too inefficient.
    // I check the value of the right end and mid. This makes
    // the performance much better (40% - 50% faster).
    fn find_pos(&self, x: &K) -> (usize, bool) {
        let len = self.ks.len();

        if len > CHECK_RIGHT_MOST_THRESHOLD && self.ks[len - 1] < *x {
            return (len, false);
        }

        if len < USE_LINEAR_THRESHOLD {
            return self.linear_search(x, 0, len);
        }

        let mid = self.ks.len() / 2;
        if self.ks[mid] < *x {
            return self.linear_search(x, mid + 1, len);
        } 
        if self.ks[mid] == *x {
            return (mid, true);
        }
        return self.linear_search_reverse(x, 0, mid);
    }

    fn linear_search(&self, x: &K, start: usize, end: usize) -> (usize, bool) {
        for i in start..end {
            if self.ks[i] < *x {
                continue;
            }
            if self.ks[i] == *x {
                return (i, true);
            }
            return (i, false);
        }
        return (end, false);
    }

    fn linear_search_reverse(&self, x: &K, start: usize, end: usize) -> (usize, bool) {
        for i in (start..end).rev() {
            if self.ks[i] < *x {
                return (i + 1, false);
            }
            if self.ks[i] == *x {
                return (i, true);
            }
        }
        return (0, false);
    }

    fn remove_right_most(&mut self, m: usize) -> (K, V, bool) {
        if self.is_leaf() {
            // 1. Leaf
            let last = self.ks.len() - 1;
            let k = self.ks.remove(last);
            let v = self.vs.remove(last);
            return (k, v, self.is_empty());
        }
        // 2. Node (remove from child)
        let last = self.ns.len() - 1;
        // self.ns.last() gets & reference. So it does not work.
        //let child = &mut self.ns[last];
        //let result = child.remove_right_most(m);
        let result = self.ns[last].remove_right_most(m);
        if result.2 {
            self.borrow_or_merge_from_left(last, m);
        }
        return (result.0, result.1, self.ks.len() < m / 2);
    }

    fn can_borrow(&self, m: usize) -> bool {
        if self.is_leaf() {
            return self.ks.len() > 1;
        }
        return self.ks.len() > m / 2;
    }

    fn borrow_left_most(&mut self) -> (K, V, Option<Node<K, V>>) {
        let k = self.ks.remove(0);
        let v = self.vs.remove(0);
        if self.is_leaf() {
            return (k, v, None);
        }
        let n = self.ns.remove(0);
        return (k, v, Some(n));
    }

    fn borrow_right_most(&mut self) -> (K, V, Option<Node<K, V>>) {
        let k = self.ks.pop().unwrap();
        let v = self.vs.pop().unwrap();
        if self.is_leaf() {
            return (k, v, None);
        }
        let n = self.ns.pop();
        return (k, v, n);
    }

    // ns[pos] <- ns[pos+1] or merge ns[pos] and ns[pos+1
    fn borrow_or_merge_from_right(&mut self, pos: usize, m: usize) {
        if self.ns[pos + 1].can_borrow(m) {
            // 1. borrow
            // remove(pos) and insert(ns) is inefficient, but
            // I need to do that. Otherwise, I have to move ks[pos]
            // and vs[pos] but Rust does not allow that.
            let pivot = (self.ks.remove(pos), self.vs.remove(pos));
            self.ns[pos].ks.push(pivot.0);
            self.ns[pos].vs.push(pivot.1);

            let borrowed = self.ns[pos + 1].borrow_left_most();

            self.ks.insert(pos, borrowed.0);
            self.vs.insert(pos, borrowed.1);

            if let Some(n) = borrowed.2 {
                self.ns[pos].ns.push(n);
            }
        } else {
            // 2. merge
            let pivot = (self.ks.remove(pos), self.vs.remove(pos));
            self.ns[pos].ks.push(pivot.0);
            self.ns[pos].vs.push(pivot.1);

            let mut removed = self.ns.remove(pos + 1);
            self.ns[pos].ks.append(&mut removed.ks);
            self.ns[pos].vs.append(&mut removed.vs);

            if !self.ns[pos].is_leaf() {
                self.ns[pos].ns.append(&mut removed.ns);
            }
        }
    }

    // ns[pos-1] -> ns[pos] or merge ns[pos-1] and ns[pos]
    fn borrow_or_merge_from_left(&mut self, pos: usize, m: usize) {
        if self.ns[pos - 1].can_borrow(m) {
            // 1. borrow
            let pivot = (self.ks.remove(pos - 1), self.vs.remove(pos - 1));
            self.ns[pos].ks.insert(0, pivot.0);
            self.ns[pos].vs.insert(0, pivot.1);

            let borrowed = self.ns[pos - 1].borrow_right_most();
            self.ks.insert(pos - 1, borrowed.0);
            self.vs.insert(pos - 1, borrowed.1);

            if let Some(n) = borrowed.2 {
                self.ns[pos].ns.insert(0, n);
            }
        } else {
            // 2. merge
            let pivot = (self.ks.remove(pos - 1), self.vs.remove(pos - 1));
            self.ns[pos - 1].ks.push(pivot.0);
            self.ns[pos - 1].vs.push(pivot.1);

            let mut removed = self.ns.remove(pos);
            self.ns[pos - 1].ks.append(&mut removed.ks);
            self.ns[pos - 1].vs.append(&mut removed.vs);

            if !self.ns[pos - 1].is_leaf() {
                self.ns[pos - 1].ns.append(&mut removed.ns);
            }
        }
    }
}

impl<K, V> fmt::Display for Node<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");
        if self.ks.len() > 0 {
            write!(f, "{}", self.ks[0]);
            for i in 1..self.ks.len() {
                write!(f, ",{}", self.ks[i]);
            }
        }
        write!(f, "]");
        Ok(())
    }
}

impl<'a, K, V> BTreeIterator<'a, K, V>
where
    K: Ord,
    K: fmt::Display,
{
    fn move_to(&mut self, x: &K) {
        loop {
            let pos = self.curr.node.find_pos(x);
            self.curr.pos = pos.0;
            if pos.1 {
                self.curr.go_child = false;
                return;
            }

            if self.curr.node.is_leaf() {
                return;
            }

            self.curr.go_child = false;
            let child = &self.curr.node.ns[pos.0];
            let mut tmp = NodeIter {
                node: child,
                pos: 0,
                go_child: false,
            };
            std::mem::swap(&mut tmp, &mut self.curr);
            self.stack.push(tmp);
        }
    }
}

impl<'a, K, V> Iterator for BTreeIterator<'a, K, V>
where
    K: Ord,
    K: fmt::Display,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.curr.node.is_leaf() {
                if self.curr.pos < self.curr.node.ks.len() {
                    // Get the right neighbor
                    let key = &self.curr.node.ks[self.curr.pos];
                    let val = &self.curr.node.vs[self.curr.pos];
                    self.curr.pos += 1;
                    return Some((key, val));
                } else {
                    match self.stack.pop() {
                        Some(x) => {
                            self.curr = x;
                            continue;
                        }
                        None => {
                            return None;
                        }
                    }
                }
            } else {
                if self.curr.go_child {
                    loop {
                        self.curr.go_child = false;
                        let child = &self.curr.node.ns[self.curr.pos];
                        let mut tmp = NodeIter {
                            node: child,
                            pos: 0,
                            go_child: false,
                        };

                        std::mem::swap(&mut tmp, &mut self.curr);
                        self.stack.push(tmp);

                        if child.is_leaf() {
                            break;
                        }
                    }
                    continue;
                } else {
                    if self.curr.pos < self.curr.node.ks.len() {
                        // Get the right neighbor
                        let key = &self.curr.node.ks[self.curr.pos];
                        let val = &self.curr.node.vs[self.curr.pos];
                        self.curr.pos += 1;
                        self.curr.go_child = true;
                        return Some((key, val));
                    } else {
                        match self.stack.pop() {
                            Some(x) => {
                                self.curr = x;
                                continue;
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                }
            }
        }
    }
}

