use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
    }
    let mut map:BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut ps:Vec<(i64, i64)> = Vec::new();
    for _ in 0..n {
        input! {
            x: i64,
            y: i64
        }
        ps.push((x, y));
    }
    for i in 0..n {
        for j in 0..n {
            let diff_x = ps[j].0 - ps[i].0;
            let diff_y = ps[j].1 - ps[i].1;
            let div = num::integer::gcd(diff_x.abs() as usize, diff_y.abs() as usize) as i64; 
            map.insert((diff_x / div, diff_y / div));
        }
    }
    println!("{:?}", map.len());
}
