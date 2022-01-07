use proconio::input;
use std::collections::BTreeSet;
use itertools::Itertools;

fn main() {
    input! {
        s:String,
        k:usize
    }
    let mut set:BTreeSet<String> = BTreeSet::new();
    let ss:Vec<char> = s.chars().collect();
    for sp in ss.iter().permutations(ss.len()) {
        let sps: String = sp.iter().map(|x| *x).collect();
        set.insert(sps);
    }
    println!("{}", set.iter().nth(k-1).unwrap());
}
