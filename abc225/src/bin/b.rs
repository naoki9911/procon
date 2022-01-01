use proconio::input;
use std::collections::BTreeSet;

fn main(){
    input! {
        n: usize,
    }
    let mut hub_v:usize = 0;
    let mut set:BTreeSet<usize> = BTreeSet::new();
    for _ in 0..n-1 {
        input!{
            a: usize,
            b: usize,
        }
        if set.contains(&a) {
            if hub_v == 0 {
                hub_v = a;
            } else if hub_v != a{
                println!("No");
                return;
            }
        }
        set.insert(a);
        if set.contains(&b) {
            if hub_v == 0 {
                hub_v = b;
            } else if hub_v != b{
                println!("No");
                return;
            }
        }
        set.insert(b);
    }

    println!("Yes");
}
