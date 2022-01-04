use proconio::input;
use std::collections::{BTreeSet};

fn main() {
    input! {
        l:usize,
        q:usize,
        cxs: [[usize; 2]; q]
    }
    let mut start_p:BTreeSet<usize> = BTreeSet::new();
    start_p.insert(0);
    start_p.insert(l);
    for cx in cxs {
        if cx[0] == 1 {
            start_p.insert(cx[1]);
        } else {
            let left = if let Some(v) = start_p.range(..cx[1]).last() {
                *v
            } else {
                0
            };
            let right = if let Some(v) = start_p.range(cx[1]..).next() {
                *v
            } else {
                l
            };
            println!("{}",right - left) 
        }
    }
}
