use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
	n: i32,
    }
    let mut names:HashMap<String, usize> = HashMap::new();
    for _ in 0..n {
        input! {
            name: String,
        }

        match names.get(&name) {
            Some(c) => names.insert(name, c+1),
            _ => names.insert(name, 1),
        };
    }

    let mut ns:Vec<(String, usize)> = names.into_iter().collect();
    ns.sort_by(|x, y| y.1.cmp(&x.1));

    println!("{}", ns[0].0);
}
