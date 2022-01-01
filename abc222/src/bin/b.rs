use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n]
    }
    let mut cnt = 0;
    for ae in a{
        if ae < p {
            cnt += 1;
        }
    }
    println!("{:?}", cnt);
}
