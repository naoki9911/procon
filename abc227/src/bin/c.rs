use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut cnt:usize = 0;
    for a in 1..=n {
        if a*a*a > n {
            break;
        }
        for b in a..=n {
            if a*b*b > n {
                break;
            }
            cnt += n/(a*b) - b + 1;
        }
    }
    println!("{:?}", cnt);
}
