use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    }
    let mut ts:Vec<VecDeque<usize>> = Vec::new();
    for _ in 0..m {
        input! {
            k: usize,
            a: [usize; k]
        }
        let mut t:VecDeque<usize> = VecDeque::new();
        for ae in a {
            t.push_back(ae);
        }
        ts.push(t);
    }
    let mut idxs_1:BTreeSet<usize> = BTreeSet::new();
    let mut idxs_2:BTreeSet<usize> = BTreeSet::new();
    for i in 0..m {
        if ts[i].len() == 0 {
            continue;
        }
        if ts[i].pop_front().unwrap() == 1 {
            idxs_1.insert(i);
        } else {
            idxs_2.insert(i) ;
        }
    }
    for _ in 0..n {
        if idxs_1.len() >= 2 {
            let mut iter = idxs_1.iter();
            let idx_1 = *iter.next().unwrap();
            let idx_2 = *iter.next().unwrap();
            idxs_1.remove(&idx_1);
            idxs_1.remove(&idx_2);
            match ts[idx_1].pop_front() {
                Some(v) => if v == 1 { idxs_1.insert(idx_1) } else { idxs_2.insert(idx_1)},
                _ => false,
            };
            match ts[idx_2].pop_front() {
                Some(v) => if v == 1 { idxs_1.insert(idx_2) } else { idxs_2.insert(idx_2)},
                _ => false,
            };
        } else if idxs_2.len() >= 2 {
            let mut iter = idxs_2.iter();
            let idx_1 = *iter.next().unwrap();
            let idx_2 = *iter.next().unwrap();
            idxs_2.remove(&idx_1);
            idxs_2.remove(&idx_2);
            match ts[idx_1].pop_front() {
                Some(v) => if v == 1 { idxs_1.insert(idx_1) } else { idxs_2.insert(idx_1)},
                None => false,
            };
            match ts[idx_2].pop_front() {
                Some(v) => if v == 1 { idxs_1.insert(idx_2) } else { idxs_2.insert(idx_2)},
                None => false,
            };
        } else {
            println!("No");
            return;
        }
    }
    if idxs_1.len() > 0 || idxs_2.len() > 0 {
        println!("No");
        return;
    }
    println!("Yes");
}
