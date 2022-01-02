use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize
    }
    let a_sum: usize = a.iter().sum();
    let mut idx = (x / a_sum) * n;
    let rest = x % a_sum;
    let mut tmp: usize = 0;
    for i in 0..n {
        if tmp > rest {
            break;
        }
        tmp += a[i];
        idx += 1;
    }
    println!("{}", idx);
}
