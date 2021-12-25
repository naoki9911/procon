use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }
    if x >= y {
        println!("0");
        return;
    }
    println!("{:?}", ((y-x-1)/10) + 1);
}
