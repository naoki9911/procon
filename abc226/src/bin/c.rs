use proconio::input;

fn build_deps(tbl: &mut Vec<bool>, arr: &Vec<(usize, Vec<usize>)>, cur_idx:usize) {
    let cur = &arr[cur_idx].1;
    tbl[cur_idx] = true;
    for a in cur {
        if tbl[a-1] {
            continue;
        }
        build_deps(tbl, arr, a-1);
    }
}

fn main() {
    input! {
        n: usize
    }

    let mut arr:Vec<(usize, Vec<usize>)> = Vec::new();
    let mut tbl:Vec<bool> = vec![false; n];
    for _ in 0..n {
        input! {
            t: usize,
            k: usize,
            a: [usize; k]
        }
        arr.push((t, a));
    }
    build_deps(&mut tbl, &arr, n-1);
    let mut res:usize = 0;
    for i in 0..n {
        if !tbl[i]  {
            continue;
        }
        res += arr[i].0;
    }
    println!("{:?}", res);
}
