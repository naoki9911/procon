use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }
    for i in ((a - 1) / c) + 1..=b / c {
        println!("{:?}", c * i);
        return;
    }
    println!("-1");
}
