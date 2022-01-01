use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut fwd:Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indeg:Vec<usize> = vec![0; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        fwd[a-1].push(b-1);
        indeg[b-1] += 1;
    }
    let mut heap:BinaryHeap<i64> = BinaryHeap::new();
    for i in 0..n {
        if indeg[i] != 0 {
            continue;
        }
        heap.push(-(i as i64));
    }

    let mut s:Vec<usize> = Vec::new();
    loop {
        if heap.len() == 0 {
            if !fwd.iter().all(|x| x.len() == 0) {
                println!("-1");
                return;
            }
            break;
        }
        let cur_v: usize = (- heap.pop().unwrap()) as usize;
        s.push(cur_v+1);
        for &dst_v in &fwd[cur_v] {
            indeg[dst_v] -= 1;
            if indeg[dst_v] == 0 {
            heap.push(-(dst_v as i64));
            }
        }
        fwd[cur_v] = Vec::new();
    }
    for sv in s {
        print!("{:?} ", sv);
    }
    println!("");
}
