use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    if s < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
