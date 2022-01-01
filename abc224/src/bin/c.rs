use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

type Point = (i64, i64);

// (x, y)
fn grad(map:&mut HashMap<Point, Point>, p1:Point, p2:Point) -> (i64, i64) {
    if p1.0 > p2.0 {
        let diff_x = p1.0 - p2.0;
        let diff_y = p1.1 - p2.1;
        if diff_x == 0 {
            return (std::i64::MAX, 0);
        }
        if diff_y == 0 {
            return (0, 0);
        }
        if map.contains_key(&(diff_x, diff_y)) {
            return *map.get(&(diff_x, diff_y)).unwrap();
        }
        let gcd = gcd(diff_x, diff_y);
        let res =  (diff_y/gcd, diff_x/gcd);
        map.insert((diff_x, diff_y), res);
        return res;
    } else {
        let diff_x = p1.0 - p2.0;
        let diff_y = p1.1 - p2.1;
        if diff_x == 0 {
            return (std::i64::MAX, 0);
        }
        if diff_y == 0 {
            return (0, 0);
        }
        if map.contains_key(&(diff_x, diff_y)) {
            return *map.get(&(diff_x, diff_y)).unwrap();
        }
        let gcd = gcd(diff_x, diff_y);
        let res =  (diff_y/gcd, diff_x/gcd);
        map.insert((diff_x, diff_y), res);
        return res;
    }
}
fn main() {
    input! {
        n: usize,
        ps: [[i64; 2]; n]
    }
    let mut cnt:usize = 0;
    let mut map:HashMap<Point, Point> = HashMap::new();
    for comb in (0..n).combinations(3) {
        let p1 = (ps[comb[0]][0], ps[comb[0]][1]);
        let p2 = (ps[comb[1]][0], ps[comb[1]][1]);
        let p3 = (ps[comb[2]][0], ps[comb[2]][1]);
        
        let grad1:(i64, i64) = grad(&mut map, p1, p2);
        let grad2:(i64, i64) = grad(&mut map, p1, p3);

        if grad1.0 == grad2.0 && grad1.1 == grad2.1 {
            continue;
        }
        cnt += 1;
    }
    println!("{:?}", cnt);
}
