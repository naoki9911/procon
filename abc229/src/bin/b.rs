use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let mut a_tmp = a;
    let mut b_tmp = b;

    while a_tmp > 0 && b_tmp > 0 {
        let a_t = a_tmp%10;
        let b_t = b_tmp%10;
        if a_t + b_t >= 10 {
            println!("Hard");
            return;
        }
        a_tmp = a_tmp/10;
        b_tmp = b_tmp/10;
    }
    println!("Easy");
}
