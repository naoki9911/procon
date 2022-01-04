use proconio::input;
use std::collections::BTreeSet;
fn main() {
    input! {
        ss: [String; 3]
    }
    let mut ans:BTreeSet<String> = BTreeSet::new();
    ans.insert("ABC".to_string());
    ans.insert("ARC".to_string());
    ans.insert("AGC".to_string());
    ans.insert("AHC".to_string());
    for s in ss {
        ans.remove(&s);
    }
    println!("{}", ans.iter().next().unwrap());
}
