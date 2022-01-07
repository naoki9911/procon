use proconio::input;

fn main() {
    input! {
        n_tmp: usize
    }
    let mut n = n_tmp;
    let mut cnt = 0;
    while n >  1 {
        n /= 2;
        cnt += 1;
    }
    println!("{:?}", cnt);
}
