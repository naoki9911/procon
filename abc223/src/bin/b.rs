use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        s: String,
    }

    let mut v:Vec<String> = Vec::new();
    let mut cs:VecDeque<char> = s.chars().collect();
    for _ in 0..s.len() {
       v.push(cs.iter().collect());
       let tmp = cs.pop_front().unwrap();
       cs.push_back(tmp);
    }

    v.sort();
    println!("{}", v[0]);
    println!("{}", v[v.len()-1]);

}
