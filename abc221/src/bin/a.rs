use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }
    let diff = a - b;
    println!("{:?}", 32_i32.pow(diff as u32));
}
