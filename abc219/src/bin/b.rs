use proconio::input;

fn main() {
   input! {
       s1: String,
       s2: String,
       s3: String,
       t: String,
    }
    let t_c:Vec<char> = t.chars().collect();
    let mut ans:String = "".to_string();
    for i in 0..t_c.len() {
        let c = t_c[i];
        if c == '1' {
            ans += &s1;
        } else if c == '2' {
            ans += &s2;
        } else {
            ans += &s3;
        }
    }
    println!("{}", ans);
}
