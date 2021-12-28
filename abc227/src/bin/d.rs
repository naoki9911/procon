use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a_v: [usize; n],
    }

    let mut ok:usize = 0;
    let mut ng:usize = 1000000000000000000/k;
    while ng - ok > 1 {
        let md = (ok + ng)/2;
        let mut sum: usize = 0;
        for a in &a_v {
            sum += a.min(&md);
        }
        if sum >= k * md {
            ok = md;
        } else {
            ng = md;
        }
    }
    println!("{:?}", ok);
}
