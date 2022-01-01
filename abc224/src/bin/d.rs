use proconio::input;
use std::collections::{VecDeque, HashSet};
 
fn v_to_s(v:&Vec<u8>) -> String {
    let mut s: String = "".to_string();
    for e in v {
        s += &e.to_string();
    }
    return s;
}
 
fn compare(v1:&Vec<u8>, v2:&Vec<u8>) -> bool {
    for (v1e, v2e) in v1.iter().zip(v2.iter()) {
        if v1e != v2e {
            return false;
        }
    }
    return true;
}
 
fn solve(edges:&Vec<Vec<u8>>, state:&Vec<u8>, empty: u8) -> i64 {
    let ans = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    if state == &ans {
        return 0;
    }
    let mut q:VecDeque<(Vec<u8>, i64)> = VecDeque::new();
    let mut done:HashSet<Vec<u8>> = HashSet::new();
    q.push_back((state.clone(), 0));
    loop {
        if q.len() == 0 {
            return -1;
        }
        let cur_state = q.pop_front().unwrap();
        done.insert(cur_state.0.clone());
        let v_empty_idx = cur_state.0.iter().position(|&x| x == empty).unwrap();
        for dst in &edges[v_empty_idx+1] {
            let mut next = cur_state.0.clone();
            next[v_empty_idx] = next[(dst-1) as usize];
            next[(dst-1) as usize] = empty;
            if next == ans {
                return cur_state.1 + 1;
            }
            if done.contains(&next) {
                continue;
            }
            q.push_back((next.clone(), cur_state.1 + 1));
        }
    }
}
fn main() {
    input! {
        m: usize,
    }
 
    let mut ve:Vec<Vec<u8>> = vec![Vec::new(); 10];
    for _ in 0..m {
        input! {
            u: u8,
            v: u8,
        }
        ve[u as usize].push(v);
        ve[v as usize].push(u);
    }
    input! {
        p: [u8; 8]
    }
    let mut empty: u8 = 45;
    let mut start:Vec<u8> = Vec::new();
    for pe in p {
        start.push(pe);
        empty -= pe;
    }
    start.push(empty);
    println!("{:?}", solve(&ve, &start, empty));
}