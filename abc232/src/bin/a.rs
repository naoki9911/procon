use proconio::input;

fn main() {
    input! {
        a: String,
    }
    let v: Vec<char> = a.chars().collect();
    let in_a = v[0].to_digit(10).unwrap();
    let in_b = v[2].to_digit(10).unwrap();

    println!("{}", in_a * in_b)
}
