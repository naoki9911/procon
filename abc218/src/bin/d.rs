use proconio::input;
use std::collections::{BTreeMap, HashSet};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        ps:[[usize; 2]; n]
    }
    let mut map_x:BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut map_y:BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut set:HashSet<(usize, usize)> = HashSet::new();
    for p in ps {
        if !map_x.contains_key(&p[0]) {
            map_x.insert(p[0], vec![p[1]]);
        } else {
            map_x.get_mut(&p[0]).unwrap().push(p[1]);
        }

        if !map_y.contains_key(&p[1]) {
            map_y.insert(p[1], vec![p[0]]);
        } else {
            map_y.get_mut(&p[1]).unwrap().push(p[0]);
        }

        set.insert((p[0], p[1]));
    }
    let mut cnt:usize = 0;
    let v:Vec<(&usize, &Vec<usize>)> = map_x.iter().collect();
    for (x, ys) in v {
        for ypp in ys.iter().combinations(2) {
            match map_y.get(ypp[0]) {
                Some(xp) => {
                    for x2 in xp {
                        if x2 == x {
                            continue;
                        }
                        if set.contains(&(*x2, *ypp[1])) {
                            cnt += 1;
                        }
                    }
                },
                None => continue,
            };
        }
    }
    println!("{:?}", cnt/2);
}
