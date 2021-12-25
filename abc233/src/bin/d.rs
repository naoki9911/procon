use proconio::input;
use std::collections::HashMap;
 
fn main() {
    input! {
        n: usize,
        k: i64,
    }
    input! {
        a: [i64; n]
    }
 
    let mut s:Vec<i64> = vec![0; n];
    let mut m:HashMap<i64, usize> = HashMap::new();
    s[0] = a[0];
    for i in 1..n {
        s[i] = s[i-1] + a[i];
    }

    let mut ans = 0;
    m.insert(0, 1);
    for i in 0..n {
        if i >= 1 {
            match m.get(&s[i-1]) {
                Some(v) => m.insert(s[i-1], v+1),
                None => m.insert(s[i-1], 1)
            };
        }
        match m.get(&(s[i] - k))  {
            Some(v) => ans = ans + v,
            None => ()
        };
    }
 
    println!("{:?}", ans);
 
}