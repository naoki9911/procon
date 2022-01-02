use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }
    if s == t {
        println!("Yes");
        return;
    }
    let tc: Vec<char> = t.chars().collect();
    for i in 0..s.len() - 1 {
        let mut tc_new = tc.clone();
        let tmp = tc_new[i];
        tc_new[i] = tc_new[i + 1];
        tc_new[i + 1] = tmp;
        let tc_new_s: String = tc_new.iter().collect();
        if s == tc_new_s {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
