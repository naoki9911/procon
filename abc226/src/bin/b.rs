use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
    }
    let mut set:BTreeSet<Vec<usize>> = BTreeSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l]
        }
        set.insert(a);
    }
    println!("{:?}", set.len());
}
