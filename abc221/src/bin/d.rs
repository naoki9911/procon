use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }
    let mut start: HashMap<usize, usize> = HashMap::new();
    let mut end: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        if start.contains_key(&a) {
            start.insert(a, start[&a] + 1);
        } else {
            start.insert(a, 1);
        }
        let e = a + b;
        if end.contains_key(&e) {
            end.insert(e, end[&e] + 1);
        } else {
            end.insert(e, 1);
        }
    }
    let mut start_v: Vec<(usize, usize)> = start.iter().map(|x| (*x.0, *x.1)).collect();
    let mut end_v: Vec<(usize, usize)> = end.iter().map(|x| (*x.0, *x.1)).collect();
    start_v.sort_by(|x, y| x.0.cmp(&y.0));
    end_v.sort_by(|x, y| x.0.cmp(&y.0));
    let mut login: Vec<usize> = vec![0usize; n + 1];
    let mut start_idx: usize = 0;
    let mut end_idx: usize = 0;
    let mut cur_day: usize = start_v[start_idx].0;
    let mut cnt: usize = start_v[start_idx].1;
    start_idx += 1;
    loop {
        if start_v.len() <= start_idx && end_v.len() <= end_idx {
            break;
        }
        if start_v.len() > start_idx && cur_day == start_v[start_idx].0 {
            cnt += start_v[start_idx].1;
            start_idx += 1;
        }
        if end_v.len() > end_idx && cur_day == end_v[end_idx].0 {
            cnt -= end_v[end_idx].1;
            end_idx += 1;
        }
        if end_v.len() <= end_idx {
            break;
        }
        let mut next_day = end_v[end_idx].0;
        if start_v.len() > start_idx {
            next_day = start_v[start_idx].0.min(end_v[end_idx].0);
        }
        login[cnt] += next_day - cur_day;
        cur_day = next_day;
    }
    for i in 1..=n {
        print!("{:?} ", login[i]);
    }
    println!("");
}
