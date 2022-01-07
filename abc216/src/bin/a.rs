use proconio::input;

fn main() {
    input! {
        a: f64
    }
    let y = ((a%1.0)*10.0) as usize;
    let x = a as usize;
    if y >= 0 && y <= 2 {
        println!("{}-", x);
    } else if y >= 3 && y <= 6 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
