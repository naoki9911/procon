use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
        s: String,
    }

    let s_v:Vec<char> = s.chars().collect();
    let mut ans:Vec<char> = Vec::new();
    for i in 1..=s.len() {
        if i < l || i > r {
            ans.push(s_v[i - 1]);
            continue
        }
        ans.push(s_v[(r - (i - l)) - 1]);
    }
    let ans_str:Vec<String> = ans.iter().map(|x| x.to_string()).collect();
    println!("{}", ans_str.join(""));
}
