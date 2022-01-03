use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }
    let s_c:Vec<char> = s.chars().collect();
    if s_c[n-1] == 'o' {
        println!("Yes");
    } else {
        println!("No");
    }
}
