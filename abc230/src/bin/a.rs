use proconio::input;

fn main() {
    input! {
	a: usize,
    }
    if a >= 42 {
        println!("AGC{:>03}", a+1);
    } else {
        println!("AGC{:>03}", a);
    }
}
