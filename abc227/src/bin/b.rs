use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        ss: [usize; n],
    }

    let mut map:BTreeMap<usize, bool> = BTreeMap::new();
    let mut a = 1;
    let mut b = 1;
    let mut s = 0;
    while s <= 1000 {
        while s <= 1000 {
            s = 4*a*b + 3*a + 3*b;
            map.insert(s, true);
            b += 1;
        }
        a += 1;
        b = 1;
        s = 4*a*b + 3*a + 3*b;
    }
    let mut cnt = 0;
    for s_e in ss {
        if let None = map.get(&s_e) {
            cnt += 1;
        }
    }
    println!("{:?}", cnt);
    
}
