use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize
    }
    let mut res = a;
    for _ in 0..(k - 1) {
        if res == n {
            res = 0;
        }
        res += 1;
    }
    println!("{:?}", res);
}
