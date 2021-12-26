use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    let s1s:Vec<char> = s1.chars().collect();
    let s2s:Vec<char> = s2.chars().collect();
    if s1s[0] == '#' {
        if s1s[1] == '#' || s2s[0] == '#' {
            println!("Yes");
            return;
        }
    }

    if s1s[1] == '#' && s2s[1] == '#' {
        println!("Yes");
        return;
    }

    if s2s[0] == '#' && s2s[1] == '#' {
        println!("Yes");
        return;
    }

    println!("No");
}
