use proconio::input;

fn main() {
    input! {
        n:usize,
        ps:[usize; n]
    }
    let mut res = vec![0_usize; n];
    for (idx, p) in (0..n).zip(ps) {
        res[p-1] = idx+1
    }
    for r in res {
        print!("{} ", r);
    }
    println!("");
}
