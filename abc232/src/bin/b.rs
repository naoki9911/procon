use proconio::input;

fn diff(a: char, b: char) -> i32 {
    let a_i = a as i32;
    let b_i = b as i32;
    let start_i = 'a' as i32;
    let end_i = 'z' as i32;
    let res = b_i - a_i;
    if res >= 0 {
        return res;
    }

    let res2 = (b_i - start_i) + (end_i - a_i) + 1;
    return res2;
}
fn main() {
    input! {
        s: String,
        t: String,
    }

    let s_v: Vec<char> = s.chars().collect();
    let t_v: Vec<char> = t.chars().collect();
    let init = diff(s_v[0], t_v[0]);
    for i in 1..s_v.len() {
        if diff(s_v[i], t_v[i]) != init {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
