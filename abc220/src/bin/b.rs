use proconio::input;

fn main() {
    input! {
        k: u32,
        a: String,
        b: String
    }
    let a_i: usize = usize::from_str_radix(&a, k).unwrap();
    let b_i: usize = usize::from_str_radix(&b, k).unwrap();
    println!("{:?}", a_i * b_i);
}
