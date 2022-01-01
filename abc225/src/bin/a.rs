use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        s: String,
    }
    let mut set: BTreeSet<String> = BTreeSet::new();
    let s_chars: Vec<char> = s.chars().into_iter().collect();
    for i in 0..3 {
        for j in 0..2 { 
            let mut s_v:Vec<char> = s_chars.clone();
            let mut change_s = s_v.remove(i).to_string();
            change_s += &s_v.remove(j).to_string();
            change_s += &s_v[0].to_string();
            set.insert(change_s.clone());
        }
    }
    println!("{:?}", set.len());
}
